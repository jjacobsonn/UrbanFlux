# UrbanFlux

[![CI](https://github.com/jjacobsonn/UrbanFlux/workflows/CI/badge.svg)](https://github.com/jjacobsonn/UrbanFlux/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

UrbanFlux is an ETL (Extract, Transform, Load) system for processing NYC 311 Service Request data. The system reads CSV files containing service request records, validates and cleans the data, removes duplicates, and loads the results into a PostgreSQL database.

## Technical Architecture

The system is implemented in Rust and uses an asynchronous processing model to handle large datasets efficiently. Key components include:

- **Extract**: Asynchronous CSV streaming with configurable chunk sizes
- **Transform**: Data validation, normalization, and deduplication
- **Load**: Bulk insertion to PostgreSQL with conflict handling
- **Database**: PostgreSQL with indexed tables and materialized views for analytics

## Features

- Streaming CSV processing with constant memory usage per chunk
- Async I/O operations using Tokio runtime
- Data validation including borough verification, coordinate bounds checking, and date validation
- In-memory deduplication based on unique keys
- Bulk database inserts with ON CONFLICT handling
- Materialized views for aggregate queries
- Structured logging via tracing framework
- Docker containerization for PostgreSQL and application

## Prerequisites

- Rust 1.80 or later
- Docker and docker-compose (for PostgreSQL)
- PostgreSQL 16 or later (if not using Docker)

## Installation

### Clone Repository

```bash
git clone https://github.com/jjacobsonn/UrbanFlux.git
cd UrbanFlux
cp .env.example .env
```

### Start PostgreSQL

```bash
make up
```

### Build Application

```bash
make build
```

### Initialize Database Schema

```bash
make db-init
```

### Run ETL Pipeline

```bash
# Process CSV file
make seed

# Or run directly with cargo
cargo run -- run --mode full --input ./testdata/sample.csv
```

### Query Results

```bash
docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux \
  -c "SELECT * FROM mv_complaints_by_day_borough LIMIT 10;"
```

## CLI Usage

### Display Help

```bash
cargo run -- --help
```

### Run ETL Commands

```bash
# Process CSV file with full mode
cargo run -- run --mode full --input ./testdata/sample.csv

# Specify chunk size
cargo run -- run --mode full --input ./testdata/sample.csv --chunk-size 50000

# Dry run (validates without database write)
cargo run -- run --mode full --input ./testdata/sample.csv --dry-run
```

### Database Commands

```bash
# Initialize schema
cargo run -- db init

# Refresh materialized views
cargo run -- db refresh-mv

# Refresh with CONCURRENTLY option
cargo run -- db refresh-mv --concurrently
```

### Report Commands

```bash
# Show last run statistics
cargo run -- report last-run
```

## Data Contract

**Input CSV Format:**
```
unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
```

**Validation Rules:**
- Borough: Must be one of BRONX, BROOKLYN, MANHATTAN, QUEENS, STATEN ISLAND
- Coordinates: Latitude [40.4, 41.2], Longitude [-74.3, -73.4]
- Timestamps: closed_date ≥ created_date
- Unique Key: Positive integer, deduplicated

**Output Schema:**
- Table: `service_requests` (with primary key on unique_key)
- Materialized Views: `mv_complaints_by_day_borough`, `mv_complaints_by_type_month`
- Control Table: `etl_watermarks` (for incremental loads)

## Testing

```bash
# Run test suite
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test validator
```

## Code Quality

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings
```

## Docker Operations

```bash
# Start PostgreSQL
docker compose up -d

# Stop services
docker compose down

# View logs
docker compose logs -f

# Remove volumes
docker compose down -v
```

## Database Schema

### Tables

**service_requests**
- Primary key: unique_key (BIGINT)
- Timestamps: created_at, closed_at, ingested_at (TIMESTAMPTZ)
- Text fields: complaint_type (required), descriptor, borough
- Coordinates: latitude, longitude (DOUBLE PRECISION)
- Constraints: Borough must be one of NYC's five boroughs

**etl_watermarks**
- Tracks ETL run metadata
- Fields: run_id, last_created_at, last_unique_key, run_mode, row counts, timestamps, status

### Indexes

- idx_service_requests_created_at: B-tree on created_at
- idx_service_requests_borough: B-tree on borough
- idx_service_requests_complaint_type: B-tree on complaint_type

### Materialized Views

**mv_complaints_by_day_borough**
- Aggregates complaints by date and borough
- Columns: complaint_date, borough, complaint_count

**mv_complaints_by_type_month**
- Aggregates complaints by month and type
- Columns: month, complaint_type, complaint_count, avg_resolution_hours

## Configuration

The system reads configuration from environment variables. Copy `.env.example` to `.env` and modify as needed:

```bash
# PostgreSQL connection
PGHOST=localhost
PGPORT=5432
PGUSER=urbanflux_user
PGPASSWORD=urbanflux_dev_password
PGDATABASE=urbanflux

# ETL settings
ETL_INPUT_PATH=./testdata/sample.csv
ETL_CHUNK_SIZE=100000
ETL_MODE=full

# Logging level
RUST_LOG=urbanflux=info,sqlx=warn
```

## Data Processing

### Extract Phase

Reads CSV files asynchronously using csv-async. Parses each row into a ServiceRequest struct with proper type conversion for timestamps, coordinates, and numeric fields.

### Transform Phase

Applies the following validations:
- Unique key must be positive integer
- Borough must be one of: BRONX, BROOKLYN, MANHATTAN, QUEENS, STATEN ISLAND
- Coordinates must be within NYC bounds (lat: 40.4-41.2, lon: -74.3 to -73.4)
- Closed date must be after created date if present
- Removes duplicate records based on unique_key

### Load Phase

Inserts validated records into PostgreSQL using individual INSERT statements with ON CONFLICT DO NOTHING to handle duplicates at the database level.

## Performance Characteristics

- Time complexity: O(N) where N is number of records
- Space complexity: O(C) where C is chunk size (default 100,000)
- Streaming processing prevents loading entire file into memory

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for contribution guidelines.

## Documentation

- [docs/QUICKSTART.md](docs/QUICKSTART.md) - Quick start guide
- [docs/USAGE.md](docs/USAGE.md) - Complete usage guide
- [docs/STYLEGUIDE.md](docs/STYLEGUIDE.md) - Code style and conventions
- [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) - Contribution process
- [docs/SECURITY.md](docs/SECURITY.md) - Security policies
- [TECH-DOC.md](TECH-DOC.md) - Detailed technical specification

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.
