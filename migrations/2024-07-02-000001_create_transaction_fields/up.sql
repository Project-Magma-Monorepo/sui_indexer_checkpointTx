-- Create the transactions table
CREATE TABLE transactions (
    tx_digest VARCHAR PRIMARY KEY,
    checkpoint_sequence_number BIGINT NOT NULL,
    sender VARCHAR NOT NULL,
    tx_kind VARCHAR NOT NULL,
    gas_budget BIGINT NOT NULL,
    gas_price BIGINT NOT NULL,
    serialized_tx JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create checkpoint_transactions table to link all fields by tx_digest
CREATE TABLE checkpoint_transactions (
    tx_digest VARCHAR PRIMARY KEY REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    transaction_digest VARCHAR NOT NULL REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    transaction_effects_digest VARCHAR,
    transaction_events_digest VARCHAR,
    input_objects_digest VARCHAR,
    output_objects_digest VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for efficient queries
CREATE INDEX idx_transactions_checkpoint ON transactions(checkpoint_sequence_number);
CREATE INDEX idx_transactions_sender ON transactions(sender);
CREATE INDEX idx_transactions_kind ON transactions(tx_kind); 