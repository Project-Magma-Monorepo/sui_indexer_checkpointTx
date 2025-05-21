use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};
use sui_indexer_alt_framework::FieldCount;
use chrono::NaiveDateTime;

use serde_json::Value;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::my_index_data)]
pub struct MyIndexData {
    pub id: String,
    pub checkpoint_sequence_number: i64,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::transactions)]
pub struct Transaction {
    pub tx_digest: String,
    pub checkpoint_sequence_number: i64,
    pub sender: String,
    pub tx_kind: Value,
    pub gas_budget: i64,
    pub gas_price: i64,
    pub serialized_tx: Value,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::checkpoint_transactions)]
pub struct CheckpointTransaction {
    pub tx_digest: String,
    pub transaction_digest: String,
    pub transaction_effects_digest: Option<String>,
    pub transaction_events_digest: Option<String>,
    pub input_objects_digest: Option<String>,
    pub output_objects_digest: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::transaction_effects)]
pub struct TransactionEffect {
    pub tx_digest: String,
    pub effects_json: Value,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::transaction_events)]
pub struct TransactionEvent {
    pub tx_digest: String,
    pub events_json: Value,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::input_objects)]
pub struct InputObjects {
    pub tx_digest: String,
    pub objects_json: Value,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, FieldCount)]
#[diesel(table_name = crate::schema::output_objects)]
pub struct OutputObjects {
    pub tx_digest: String,
    pub objects_json: Value,
    pub created_at: Option<NaiveDateTime>,
}

impl Transaction {
    pub fn new(
        tx_digest: String,
        checkpoint_sequence_number: i64,
        sender: String,
        tx_kind: Value,
        gas_budget: i64,
        gas_price: i64,
        serialized_tx: Value,
    ) -> Self {
        Self {
            tx_digest,
            checkpoint_sequence_number,
            sender,
            tx_kind,
            gas_budget,
            gas_price,
            serialized_tx,
            created_at: None,
        }
    }
}

impl CheckpointTransaction {
    pub fn new(
        tx_digest: String,
    ) -> Self {
        Self {
            tx_digest: tx_digest.clone(),
            transaction_digest: tx_digest,
            transaction_effects_digest: None,
            transaction_events_digest: None,
            input_objects_digest: None,
            output_objects_digest: None,
            created_at: None,
        }
    }
}

// No longer needed since we're not dealing with defaults
// #[derive(Debug, Clone, Insertable)]
// #[diesel(table_name = crate::schema::my_index_data)]
// pub struct NewMyIndexData {
//     pub id: String,
//     pub checkpoint_sequence_number: i64,
// } 