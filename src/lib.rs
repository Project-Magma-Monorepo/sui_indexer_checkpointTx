use std::sync::Arc;
use std::collections::HashMap;
use url::Url;
use anyhow::anyhow;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::info;

pub mod schema;
pub mod models;

// Embed the migrations in the library
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use sui_indexer_alt_framework::{
    cluster::{self, IndexerCluster}, 
    db, 
    pipeline::{
        concurrent::{ConcurrentConfig, Handler as ConcurrentHandler}, 
        Processor
    }, 
    types::full_checkpoint_content::CheckpointData, 
    FieldCount, 
    Result
};

use sui_types::{
    base_types::{ObjectID, SuiAddress}, 
    transaction::TransactionDataAPI
};

use crate::models::MyIndexData;

// Enum to specify which fields to index
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexField {
    Transaction,
    Effects,
    Events,
    InputObjects,
    OutputObjects,
}

// Type for callback functions
pub type IndexCallback = Box<dyn Fn(&CheckpointData) -> Result<Vec<MyIndexData>> + Send + Sync>;

pub struct SuiIndexer {
    package_filter: Option<SuiAddress>,
    field_filters: Vec<IndexField>,
    field_callbacks: HashMap<IndexField, IndexCallback>,
}

impl SuiIndexer {
    pub fn new() -> Self {
        Self {
            package_filter: None,
            field_filters: Vec::new(),
            field_callbacks: HashMap::new(),
        }
    }

    pub fn set_filter_package(&mut self, package: SuiAddress) {
        self.package_filter = Some(package);
    }

    pub fn set_filter_fields(&mut self, fields: Vec<IndexField>) {
        self.field_filters = fields;
    }

    pub fn set_filter_callback_for_field(
        &mut self,
        field: IndexField,
        callback: impl Fn(&CheckpointData) -> Result<Vec<MyIndexData>> + Send + Sync + 'static,
    ) {
        self.field_callbacks.insert(field, Box::new(callback));
    }

    pub async fn start(
        self,
        database_url: Url,
        cluster_args: cluster::Args,
    ) -> Result<()> {
        let package_filter = self.package_filter
            .ok_or_else(|| anyhow!("Package filter not set"))?;
            
        let pipeline = IndexerPipeline {
            field_filters: self.field_filters,
            package_filter,
            callbacks: self.field_callbacks,
        };

        // Initialize the cluster with our migrations
        let mut indexer = IndexerCluster::new(
            database_url,
            cluster_args,
            Some(&crate::MIGRATIONS),
        ).await?;
        
        let _ = indexer.concurrent_pipeline(pipeline, ConcurrentConfig::default()).await;
        
        // Run the indexer
        let _ = indexer.run().await?.await;
        
        Ok(())
    }
}

// Concrete Pipeline implementation for MyIndexData
pub struct IndexerPipeline {
    field_filters: Vec<IndexField>,
    package_filter: SuiAddress,
    callbacks: HashMap<IndexField, IndexCallback>,
}

impl IndexerPipeline {
    fn check_package(&self, package_id: &ObjectID) -> bool {
        package_id.to_string() == self.package_filter.to_string()
    }
}

impl Processor for IndexerPipeline {
    const NAME: &'static str = "indexer_pipeline";
    type Value = models::Transaction;

    fn process(&self, checkpoint: &Arc<CheckpointData>) -> Result<Vec<Self::Value>> {
        info!("Processing checkpoint: {}", checkpoint.checkpoint_summary.sequence_number);
        info!("Target package: {}", self.package_filter);
        info!("Number of transactions in checkpoint: {}", checkpoint.transactions.len());
        
        let mut results = Vec::new();
        
        for (i, tx) in checkpoint.transactions.iter().enumerate() {
            let tx_digest = tx.transaction.digest().to_string();
            let sender = tx.transaction.transaction_data().sender().to_string();
            let move_calls = tx.transaction.transaction_data().move_calls();
            
            info!("Examining transaction {} of {}: digest={}, sender={}",
                  i+1, checkpoint.transactions.len(), tx_digest, sender);
            
            // Check if any move call in this transaction uses our package
            if move_calls.is_empty() {
                info!("  Transaction has no move calls, skipping");
                continue;
            }
            
            info!("  Transaction has {} move calls", move_calls.len());
            
            let mut package_matched = false;
            
            for (j, call) in move_calls.iter().enumerate() {
                let package_id = &call.0;
                let module_name = &call.1;
                let function_name = &call.2;
                
                info!("  Move call {}: {}::{}::{}", j+1, package_id, module_name, function_name);
                
                // Check if this call uses our package
                if self.check_package(package_id) {
                    info!("  MATCH FOUND! Transaction {} uses target package in module {}, function {}", 
                          tx_digest, module_name, function_name);
                    package_matched = true;
                    break;
                }
            }
            
            if !package_matched {
                info!("  No matching package found in this transaction, skipping");
                continue;
            }

            // Once we find a matching package, create a transaction record
            let tx_data = tx.transaction.transaction_data();
            let kind = format!("{:?}", tx_data.kind());
            let checkpoint_seq = checkpoint.checkpoint_summary.sequence_number as i64;
            
            // Serialize the full transaction for storage
            let serialized_tx = serde_json::to_value(&tx.transaction).unwrap_or_default();
            
            // Create the transaction record
            let transaction_record = models::Transaction::new(
                tx_digest,
                checkpoint_seq,
                sender,
                kind,
                tx_data.gas_budget() as i64,
                tx_data.gas_price() as i64,
                serialized_tx
            );
            
            results.push(transaction_record);
        }
        
        info!("Finished processing checkpoint {}, found {} matching transactions", 
              checkpoint.checkpoint_summary.sequence_number, results.len());
        
        Ok(results)
    }
}

#[async_trait::async_trait]
impl ConcurrentHandler for IndexerPipeline {
    async fn commit(values: &[Self::Value], conn: &mut db::Connection<'_>) -> Result<usize> {
        if values.is_empty() {
            return Ok(0);
        }
        
        // Insert transactions
        info!("Inserting {} transaction records", values.len());
        
        use crate::schema::transactions;
        
        let inserted = diesel::insert_into(transactions::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to insert transaction records: {}", e))?;
        
        info!("Successfully inserted {} transaction records", inserted);
        Ok(inserted)
    }
}



