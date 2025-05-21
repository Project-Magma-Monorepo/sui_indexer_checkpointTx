use clap::Parser;
use sui_indexer_alt_framework::{
    cluster::{self, IndexerCluster}, Result
};
use sui_types::{base_types::SuiAddress};
use url::Url;

use sui_indexer_generic::{
    SuiIndexer, 
    IndexField,
    models::Transaction
};

// No need to redefine MIGRATIONS here since it's defined in lib.rs

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(
        long,
        default_value = "postgres://postgres:postgres@localhost:5432/sui_indexer_generic"
    )]
    database_url: Url,

    #[clap(flatten)]
    cluster_args: cluster::Args,

    #[clap(long)]
    package_address: SuiAddress,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    
    let args = Args::parse();
    
    let mut indexer = SuiIndexer::new();
    
    // Set the package address to track
    indexer.set_filter_package(args.package_address);
    
    // We want to track both transactions and their effects
    indexer.set_filter_fields(vec![
        IndexField::Transaction,
        IndexField::Effects,
    ]);
    
    // Start indexing with the database URL and cluster args
    indexer.start(args.database_url, args.cluster_args).await?;
    
    Ok(())
} 