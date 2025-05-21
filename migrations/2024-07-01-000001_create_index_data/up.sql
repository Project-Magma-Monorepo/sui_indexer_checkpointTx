-- Create a table for MyIndexData
CREATE TABLE my_index_data (
    id VARCHAR NOT NULL PRIMARY KEY,
    checkpoint_sequence_number BIGINT NOT NULL
);

-- Create an index on checkpoint_sequence_number for faster queries
CREATE INDEX idx_my_index_data_checkpoint ON my_index_data(checkpoint_sequence_number); 