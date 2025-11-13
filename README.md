# UrbanFlux

[![CI](https://github.com/jjacobsonn/UrbanFlux/workflows/CI/badge.svg)](https://github.com/jjacobsonn/UrbanFlux/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Professional NYC 311 Service Request ETL Pipeline**

A production-grade, type-safe ETL system built in Rust for processing NYC 311 service requests. Features streaming CSV ingestion, batch processing, materialized views, and comprehensive observability.

---

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/jjacobsonn/UrbanFlux.git
cd UrbanFlux

# 2. Run automated setup
chmod +x setup.sh
./setup.sh
```

Select option `1` for Full Installation. The script automatically:
- Checks dependencies (Docker, Rust, etc.)
- Builds the project
- Starts PostgreSQL
- Runs migrations
- Loads test data
- Verifies everything works

---

## Interactive Menu

For easy management:

```bash
./setup.sh
```

Available operations:
```
1)  Full Installation          7)  Run Tests
2)  Build Project              8)  Health Check
3)  Start Docker               9)  Query Data
4)  Run Migrations            10)  Stop Services
5)  Load Test Data            11)  Reset Everything
6)  Refresh Views             12)  Show Logs
```

---

## System Requirements

### Required
- **Docker** (v20.10+) - [Install Docker](https://docs.docker.com/get-docker/)
- **Rust** (v1.80+) - Install via:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

The setup script checks all dependencies automatically.

---

## Architecture

### Domain-Driven Design

```
src/
├── domain/          # Business logic & entities
│   ├── models.rs    # ServiceRequest, Borough, Coordinates
│   └── validation.rs # Validation rules
├── extract/         # CSV streaming & parsing
│   ├── parser.rs    # Multi-format datetime parsing
│   └── stream.rs    # Chunked streaming (O(chunk_size) memory)
├── transform/       # Data transformation
│   ├── deduplicator.rs  # HashSet-based dedup
│   └── processor.rs     # Validation pipeline
├── database/        # Repository pattern
│   ├── connection.rs    # Connection pool
│   ├── repository.rs    # Batch operations
│   └── watermark.rs     # ETL tracking
├── config.rs        # Configuration
├── error.rs         # Custom errors (thiserror)
├── logging.rs       # Structured logging (tracing)
└── main.rs          # CLI (clap)
```

### Key Features

- **Memory Efficient**: Streams CSV in configurable chunks (default 100k rows)
- **Batch Processing**: Bulk INSERTs with 1000 rows/batch
- **Idempotent**: Watermark tracking prevents duplicate processing
- **Type Safe**: Leverages Rust's type system
- **Observable**: Structured JSON logging
- **Tested**: 17 unit tests covering all critical paths

---

## Database Schema

### Tables

**service_requests** - Main fact table
- unique_key (PK), created_at, closed_at, due_date
- complaint_type, descriptor, borough
- latitude, longitude with NYC bounds validation
- Indexes on borough, created_at, complaint_type

**etl_watermarks** - ETL run tracking
- run_id (UUID), run_mode (full/incremental)
- rows_processed, rows_inserted, rows_duplicated, rows_rejected
- status (running/completed/failed)

### Materialized Views

**mv_complaints_by_day_borough** - Daily borough aggregates  
**mv_complaints_by_type_month** - Monthly complaint trends

Both views support concurrent refresh operations via unique indexes.

---

## CLI Usage

### ETL Commands

```bash
# Full ETL run
cargo run --release -- run --mode full --input data/311_requests.csv

# Incremental run
cargo run --release -- run --mode incremental --input data/311_requests.csv

# Custom chunk size
cargo run --release -- run --mode full --input data/311_requests.csv --chunk-size 50000

# Dry run (validate without writing)
cargo run --release -- run --mode full --input data/311_requests.csv --dry-run
```

### Database Management

```bash
# Run migrations
cargo run --release -- db migrate

# Check health
cargo run --release -- db health

# Refresh materialized views
cargo run --release -- db refresh-mv --concurrently
```

### Reporting

```bash
# Show last ETL run statistics
cargo run --release -- report last-run
```

---

## Testing

```bash
# Unit tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test domain::validation

# Integration testing via setup script
./setup.sh
# Select option 7 (Run Tests)
```

---

## Environment Configuration

`.env` file (auto-created by setup script):

```bash
PGHOST=localhost
PGPORT=5432
PGUSER=urbanflux_user
PGPASSWORD=urbanflux_dev_password
PGDATABASE=urbanflux

ETL_CHUNK_SIZE=100000
ETL_MODE=full
RUST_LOG=info
LOG_FORMAT=pretty
```

---

## Performance

- **CSV Parsing**: ~500k rows/second
- **Validation**: ~800k rows/second  
- **Bulk Insert**: ~100k rows/second
- **Memory**: O(chunk_size) - typically 100-200 MB

### Optimization Tips

```bash
# Large files: increase chunk size
cargo run --release -- run --mode full --input large.csv --chunk-size 500000

# Always use release build for production
cargo build --release

# Refresh views concurrently to avoid locking
cargo run --release -- db refresh-mv --concurrently
```

---

## Troubleshooting

### Dependencies Missing
```bash
./setup.sh  # Select option 1 for guided installation
```

### PostgreSQL Issues
```bash
docker compose logs postgres
docker compose restart postgres
```

### Build Issues
```bash
cargo clean
cargo build --release
```

### Complete Reset
```bash
./setup.sh  # Select option 11 (Complete Reset)
```

---

## Tech Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Language | Rust | 1.80+ |
| Runtime | Tokio | 1.42 |
| Database | PostgreSQL | 16 |
| SQL Toolkit | SQLx | 0.8 |
| CLI | Clap | 4.5 |
| CSV | csv-async | 1.3 |
| Logging | Tracing | 0.1 |
| Errors | thiserror | 2.0 |

---

## Documentation

- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- [docs/TECH-DOC.md](docs/TECH-DOC.md) - Technical specification
- [docs/USAGE.md](docs/USAGE.md) - Complete usage guide
- [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) - Contribution guidelines
- [docs/SECURITY.md](docs/SECURITY.md) - Security policies

---

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests if applicable
4. Run `cargo fmt` and `cargo clippy`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Submit a pull request

---

## License

MIT License - See LICENSE file for details

---

**Professional ETL built with Rust**
