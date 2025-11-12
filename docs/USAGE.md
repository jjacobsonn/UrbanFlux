# UrbanFlux Usage Guide

## Overview

UrbanFlux provides a complete ETL pipeline for processing NYC 311 service request data. The system extracts data from CSV files, validates and transforms records, and loads them into PostgreSQL for analytics.

## Prerequisites

### Option 1: With Docker (Recommended)
```bash
# Start Docker Desktop
open -a Docker

# Start PostgreSQL
cd UrbanFlux
make up

# Wait ~10 seconds for PostgreSQL to initialize
```

### Option 2: Local PostgreSQL
```bash
# Install PostgreSQL (if not installed)
brew install postgresql@16
brew services start postgresql@16

# Create database and user
psql postgres
```
```sql
CREATE USER urbanflux_user WITH PASSWORD 'urbanflux_dev_password';
CREATE DATABASE urbanflux OWNER urbanflux_user;
\q
```

## Quick Start

### 1. Initialize Database Schema

```bash
cargo run -- db init
```

**Expected Output:**
```
Initializing database schema...
Database schema initialized successfully!
```

### 2. Test with Dry Run

Validate the pipeline without writing to the database:

```bash
cargo run -- run \
  --input ./testdata/sample.csv \
  --chunk-size 5 \
  --dry-run
```

**Expected Output:**
```
DRY RUN MODE - No database writes will occur

Extracting data from CSV...
Extracted 10 records in 2 chunks
Processing chunk 1/2
Processing chunk 2/2

ETL Summary:
  Total extracted: 10
  Total rejected:  0
  Would load:      10

ETL pipeline completed successfully!
```

### 3. Load Data into PostgreSQL

Execute the full ETL pipeline:

```bash
cargo run -- run \
  --input ./testdata/sample.csv \
  --chunk-size 100
```

**Expected Output:**
```
Extracting data from CSV...
Extracted 10 records in 1 chunk
Processing chunk 1/1

ETL Summary:
  Total extracted: 10
  Total rejected:  0
  Total loaded:    10
  Records in DB:   10

ETL pipeline completed successfully!
```

## Command Reference

### ETL Pipeline Execution

```bash
# Full load with default settings
cargo run -- run --input <path-to-csv>

# Custom chunk size for memory optimization
cargo run -- run --input <path> --chunk-size 50000

# Dry run mode (validation only, no database writes)
cargo run -- run --input <path> --dry-run

# Incremental mode (future feature)
cargo run -- run --mode incremental --input <path>
```

### Database Management

```bash
# Initialize schema (tables, indexes, materialized views)
cargo run -- db init

# Refresh materialized views
cargo run -- db refresh-mv

# Refresh with CONCURRENTLY option (non-blocking)
cargo run -- db refresh-mv --concurrently
```

### Reporting

```bash
# Display statistics from last ETL run
cargo run -- report last-run
```

## Querying Data

After loading data, connect to PostgreSQL and run queries:

```bash
# Connect to database
psql -h localhost -U urbanflux_user -d urbanflux

# Count total records
SELECT COUNT(*) FROM service_requests;

# View recent complaints
SELECT 
    unique_key, 
    created_at, 
    complaint_type, 
    borough 
FROM service_requests 
ORDER BY created_at DESC 
LIMIT 10;

# Aggregate complaints by borough
SELECT 
    borough, 
    COUNT(*) as count 
FROM service_requests 
GROUP BY borough 
ORDER BY count DESC;

# Query materialized view
SELECT * FROM mv_complaints_by_day_borough LIMIT 10;
```

## Complete Workflow Example

```bash
# 1. Clean existing environment
make teardown

# 2. Start PostgreSQL
make up

# 3. Initialize database schema
cargo run -- db init

# 4. Test with dry-run mode
cargo run -- run --input ./testdata/sample.csv --dry-run

# 5. Load data into database
cargo run -- run --input ./testdata/sample.csv

# 6. Verify data loaded
cargo run -- report last-run

# 7. Refresh analytics views
cargo run -- db refresh-mv

# 8. Query results
docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux \
  -c "SELECT borough, COUNT(*) FROM service_requests GROUP BY borough;"
```

## CSV Data Format

UrbanFlux expects CSV files with the following schema:

```csv
unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
```

### Column Specifications

- **unique_key**: Positive integer (required)
- **created_date**: Timestamp in format `YYYY-MM-DD HH:MM:SS` (required)
- **closed_date**: Timestamp or empty (optional)
- **complaint_type**: Non-empty text (required)
- **descriptor**: Text description (optional)
- **borough**: Must be one of: BRONX, BROOKLYN, MANHATTAN, QUEENS, STATEN ISLAND (optional)
- **latitude**: Float between 40.4 and 41.2 (optional)
- **longitude**: Float between -74.3 and -73.4 (optional)

### Example CSV Data
```csv
unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
100001,2025-01-15 09:30:00,2025-01-15 11:00:00,Noise,Loud Music,MANHATTAN,40.7580,-73.9855
100002,2025-01-15 10:00:00,,Street Condition,Pothole,BROOKLYN,40.6782,-73.9442
```

## Testing

### Run Test Suite
```bash
# Execute all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test validator

# Run specific test function
cargo test test_validate_borough
```

### Code Quality Checks
```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## Configuration

Edit `.env` file to customize behavior:

```bash
```bash
# PostgreSQL connection
PGHOST=localhost
PGPORT=5432
PGUSER=urbanflux_user
PGPASSWORD=your_password
PGDATABASE=urbanflux

# ETL settings
ETL_CHUNK_SIZE=100000
ETL_MODE=full

# Logging level
RUST_LOG=urbanflux=info,sqlx=warn
```

## Troubleshooting

### Database Connection Failure
```

---

## 🐛 Troubleshooting

### "Failed to connect to PostgreSQL"
```bash
# Verify PostgreSQL is running
docker ps

# Or check local service
brew services list

# Test connection manually
psql -h localhost -U urbanflux_user -d urbanflux
```

### File Not Found Error
```bash
# Use absolute path
cargo run -- run --input $(pwd)/testdata/sample.csv

# Or relative path from project root
cargo run -- run --input ./testdata/sample.csv
```

### CSV Parsing Errors

```bash
# Validate CSV structure
head -5 your_file.csv

# Verify headers match expected schema
# Required: unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
```

### Port Conflict (5432 already in use)

```bash
# Stop existing PostgreSQL service
brew services stop postgresql@16

# Or modify port in docker-compose.yml
ports:
  - "5433:5432"  # Use port 5433 on host
```

## Performance Optimization

### Large Dataset Processing

```bash
# Increase chunk size for higher throughput
cargo run -- run --input large_file.csv --chunk-size 500000

# Reduce logging overhead
RUST_LOG=urbanflux=warn cargo run -- run --input large_file.csv

# Build in release mode for production
cargo build --release
./target/release/urbanflux run --input large_file.csv
```

### Database Tuning

For bulk loading operations, adjust PostgreSQL settings in `docker-compose.yml`:

```yaml
environment:
  POSTGRES_SHARED_BUFFERS: "256MB"
  POSTGRES_WORK_MEM: "16MB"
  POSTGRES_MAINTENANCE_WORK_MEM: "128MB"
```

## Features

### Implemented

- Streaming CSV extraction with configurable chunk sizes
- Data validation and cleaning
- Deduplication by unique_key
- Bulk insertion to PostgreSQL
- Error handling and structured logging
- Schema initialization with indexes and materialized views
- Dry-run mode for testing

### Data Quality Validations

- Borough must be one of NYC's five boroughs
- Coordinates must fall within NYC bounding box (lat: 40.4-41.2, lon: -74.3 to -73.4)
- Closed date must be after or equal to created date
- Text fields are cleaned and normalized

## Next Steps

1. **Obtain NYC 311 Dataset**
   - Download from: https://data.cityofnewyork.us/Social-Services/311-Service-Requests-from-2010-to-Present/erm2-nwe9
   - Place file in `testdata/` directory

2. **Process Production Data**
   ```bash
   cargo run --release -- run --input testdata/311_data.csv --chunk-size 100000
   ```

3. **Build Analytics**
   - Query materialized views for aggregated insights
   - Create dashboards using BI tools
   - Generate reports on service request patterns
