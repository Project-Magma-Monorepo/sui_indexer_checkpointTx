-- Create the transactions table
CREATE TABLE transactions (
    tx_digest VARCHAR PRIMARY KEY,
    checkpoint_sequence_number BIGINT NOT NULL,
    sender VARCHAR NOT NULL,
    tx_kind JSONB NOT NULL,
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

-- Create transaction_effects table to store effects_json
CREATE TABLE transaction_effects (
    tx_digest VARCHAR PRIMARY KEY REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    effects_json JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create transaction_events table to store events data
CREATE TABLE transaction_events (
    tx_digest VARCHAR PRIMARY KEY REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    events_json JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create input_objects table to store all input objects as a JSON array
CREATE TABLE input_objects (
    tx_digest VARCHAR PRIMARY KEY REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    objects_json JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create output_objects table to store all output objects as a JSON array
CREATE TABLE output_objects (
    tx_digest VARCHAR PRIMARY KEY REFERENCES transactions(tx_digest) ON DELETE CASCADE,
    objects_json JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for efficient queries
CREATE INDEX idx_transactions_checkpoint ON transactions(checkpoint_sequence_number);
CREATE INDEX idx_transactions_sender ON transactions(sender);
CREATE INDEX idx_transactions_kind ON transactions USING GIN (tx_kind);
CREATE INDEX idx_input_objects_tx_digest ON input_objects(tx_digest);
CREATE INDEX idx_output_objects_tx_digest ON output_objects(tx_digest);