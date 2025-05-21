# Sui Indexer Generic

A powerful and flexible indexer for the Sui blockchain that allows you to track and store transactions related to specific packages. This tool is particularly useful for creating Dune Analytics dashboards and tracking on-chain activity for specific smart contracts.

## Features

- Track transactions for specific Sui packages
- Store detailed transaction data including:
  - Transaction details
  - Transaction effects
  - Transaction events
  - Input objects
  - Output objects
- Start indexing from any checkpoint
- Configurable database storage
- Concurrent processing for better performance
- JSON-based storage format for flexible querying

## Prerequisites

Before running the indexer, make sure you have the following installed:

1. Rust (latest stable version)
2. PostgreSQL 15
3. Diesel CLI
4. Homebrew (for macOS users)

## Installation

1. Install PostgreSQL 15:
```bash
brew install postgresql@15
```

2. Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

3. Clone the repository:
```bash
git clone <repository-url>
cd sui_indexer_checkpointTx
```

4. Set up the database:
```bash
# Create a .env file with your database URL
echo "DATABASE_URL=postgres://username:password@localhost/sui_indexer" > .env

# Run migrations
diesel migration run
```

## Usage

To run the indexer, use the following command:

```bash
RUST_LOG=info DYLD_LIBRARY_PATH="/opt/homebrew/opt/postgresql@15/lib:$DYLD_LIBRARY_PATH" cargo run --release -- \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --first-checkpoint <checkpoint_number> \
  --skip-watermark \
  --package-address <package_address>
```

### Command Parameters

- `--remote-store-url`: The Sui network checkpoint URL (mainnet/testnet)
- `--first-checkpoint`: The checkpoint number to start indexing from
- `--skip-watermark`: Skip the watermark check to start from the specified checkpoint
- `--package-address`: The address of the package to track

### Example

```bash
RUST_LOG=info DYLD_LIBRARY_PATH="/opt/homebrew/opt/postgresql@15/lib:$DYLD_LIBRARY_PATH" cargo run --release -- \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --first-checkpoint 138216332 \
  --skip-watermark \
  --package-address 0x3864c7c59a4889fec05d1aae4bc9dba5a0e0940594b424fbed44cb3f6ac4c032
```

## Database Schema

The indexer stores data in the following tables, with all complex data structures stored in JSON format for maximum flexibility:

1. `transactions`: Stores transaction details including:
   - Transaction digest
   - Checkpoint sequence number
   - Sender address
   - Transaction kind (as JSON)
   - Gas budget and price
   - Full transaction data (as JSON)

2. `transaction_effects`: Stores transaction effects as JSON, including:
   - Created objects
   - Mutated objects
   - Deleted objects
   - Gas effects
   - Events

3. `transaction_events`: Stores transaction events as JSON, including:
   - Event types
   - Event data
   - Package information

4. `input_objects`: Stores input objects as JSON, including:
   - Object references
   - Object versions
   - Object data

5. `output_objects`: Stores output objects as JSON, including:
   - Created objects
   - Mutated objects
   - Object versions
   - Object data

Each table includes a `tx_digest` field to link related records together.

### Implementation Details

The indexer uses the Sui Alt Framework's checkpoint content structure for processing transactions. For detailed implementation information, refer to the `full_checkpoint_content.rs` file in the [sui-alt-framework repository](https://github.com/your-org/sui-alt-framework). This file contains the core data structures and processing logic for handling checkpoint data.

## Use Cases

This indexer is particularly useful for:

1. Creating Dune Analytics dashboards
2. Tracking specific smart contract activity
3. Analyzing transaction patterns
4. Building custom analytics tools
5. Monitoring on-chain events
6. Complex JSON-based queries on transaction data

## Integration Options

### Analytics Solutions

1. **Self-Hosted Analytics**
   - Run your own indexer instance
   - Full control over data processing and storage
   - Custom querying capabilities
   - Real-time data access
   - Perfect for projects requiring high data privacy

2. **Dune Analytics Integration**
   - Connect your indexed data to Dune Analytics
   - Create custom dashboards
   - Share analytics with your community
   - Leverage Dune's powerful querying capabilities
   - Real-time data visualization

3. **Nautilus Framework** (Coming Soon)
   - On-chain verifiable computation for analytics
   - Trustless data processing and verification
   - Cross-project analytics verification
   - Community-driven analytics validation
   - Transparent computation proofs

### Getting Started with Analytics

1. **Self-Hosted Setup**
   ```bash
   # Clone and set up the indexer
   git clone <repository-url>
   cd sui_indexer_checkpointTx
   
   # Configure your database
   echo "DATABASE_URL=postgres://username:password@localhost/sui_indexer" > .env
   
   # Run the indexer
   cargo run --release -- --package-address <your-package-address>
   ```

2. **Dune Analytics Integration**
   - Export your indexed data to Dune Analytics
   - Use the JSON format for flexible querying
   - Create custom SQL queries
   - Build interactive dashboards

3. **Nautilus Framework**
   - Leverage on-chain verifiable computation
   - Verify analytics results trustlessly
   - Share and validate analytics across projects
   - Build trust in your analytics pipeline

## Customization

The indexer is designed to be flexible and extensible. If you need to track specific data for your product or use case, you have two options:

1. **Modify the Source Code**: You can modify the logic in `lib.rs` to add custom data processing or filtering. The code is structured to make it easy to add new functionality while maintaining the existing features.

2. **Request Implementation**: If you prefer not to modify the code yourself, you can:
   - Open an issue on the repository
   - Describe your specific requirements
   - Our team will help implement the necessary changes

Common customization requests include:
- Adding specific event type filtering
- Custom transaction data processing
- Additional object tracking
- Specialized data aggregation
- Custom JSON field extraction

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license here] 