// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    checkpoint_transactions (tx_digest) {
        tx_digest -> Varchar,
        transaction_digest -> Varchar,
        transaction_effects_digest -> Nullable<Varchar>,
        transaction_events_digest -> Nullable<Varchar>,
        input_objects_digest -> Nullable<Varchar>,
        output_objects_digest -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    input_objects (tx_digest) {
        tx_digest -> Varchar,
        objects_json -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    my_index_data (id) {
        id -> Varchar,
        checkpoint_sequence_number -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    output_objects (tx_digest) {
        tx_digest -> Varchar,
        objects_json -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    transaction_effects (tx_digest) {
        tx_digest -> Varchar,
        effects_json -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    transaction_events (tx_digest) {
        tx_digest -> Varchar,
        events_json -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    transactions (tx_digest) {
        tx_digest -> Varchar,
        checkpoint_sequence_number -> Int8,
        sender -> Varchar,
        tx_kind -> Jsonb,
        gas_budget -> Int8,
        gas_price -> Int8,
        serialized_tx -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(input_objects -> transactions (tx_digest));
diesel::joinable!(output_objects -> transactions (tx_digest));
diesel::joinable!(transaction_effects -> transactions (tx_digest));
diesel::joinable!(transaction_events -> transactions (tx_digest));

diesel::allow_tables_to_appear_in_same_query!(
    checkpoint_transactions,
    input_objects,
    my_index_data,
    output_objects,
    transaction_effects,
    transaction_events,
    transactions,
);
