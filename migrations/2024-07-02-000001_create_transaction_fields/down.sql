-- Drop tables in the reverse order of creation to respect foreign key constraints
DROP TABLE IF EXISTS checkpoint_transactions;
DROP TABLE IF EXISTS transactions; 