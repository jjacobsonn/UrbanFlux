# UrbanFlux - Complete Usage Guide

## 🚀 The ETL Pipeline is Now Functional!

UrbanFlux now has a **working ETL pipeline** that can:
- ✅ Extract data from CSV files (streaming, chunked)
- ✅ Transform and validate records (borough, coordinates, dates)
- ✅ Load data into PostgreSQL (bulk insert with deduplication)
- ✅ Manage database schema and materialized views
- ✅ Run in dry-run mode for testing

---

## 📋 Prerequisites

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

---

## 🎯 Quick Start (3 Steps)

### 1. Initialize Database Schema

```bash
cargo run -- db init
```

**Output:**
```
🔧 Initializing database schema...
✅ Database schema initialized successfully!
```

### 2. Run ETL Pipeline (Dry Run)

Test without database writes:

```bash
cargo run -- run \
  --input ./testdata/sample.csv \
  --chunk-size 5 \
  --dry-run
```

**Output:**
```
🔍 DRY RUN MODE - No database writes will occur

📥 Extracting data from CSV...
✅ Extracted 10 records in 2 chunks
🔄 Processing chunk 1/...
🔄 Processing chunk 2/...

📊 ETL Summary:
  Total extracted: 10
  Total rejected:  0
  Would load:      10

✨ ETL pipeline completed successfully!
```

### 3. Run Full ETL Pipeline

Load data into the database:

```bash
cargo run -- run \
  --input ./testdata/sample.csv \
  --chunk-size 100
```

**Output:**
```
📥 Extracting data from CSV...
✅ Extracted 10 records in 1 chunks
🔄 Processing chunk 1/...

📊 ETL Summary:
  Total extracted: 10
  Total rejected:  0
  Total loaded:    10
  Records in DB:   10

✨ ETL pipeline completed successfully!
```

---

## 🛠️ Complete Command Reference

### Run ETL Pipeline

```bash
# Full load (default mode)
cargo run -- run --input <path-to-csv>

# With custom chunk size
cargo run -- run --input <path> --chunk-size 50000

# Dry run (no database writes)
cargo run -- run --input <path> --dry-run

# Incremental mode (future feature)
cargo run -- run --mode incremental --input <path>
```

### Database Operations

```bash
# Initialize schema (tables, indexes, materialized views)
cargo run -- db init

# Refresh materialized views
cargo run -- db refresh-mv

# Refresh concurrently (non-blocking)
cargo run -- db refresh-mv --concurrently
```

### Reporting

```bash
# Show last run summary
cargo run -- report last-run
```

---

## 📊 Query Your Data

After loading data, query PostgreSQL:

```bash
# Connect to database
psql -h localhost -U urbanflux_user -d urbanflux

# Count records
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

# Complaints by borough
SELECT 
    borough, 
    COUNT(*) as count 
FROM service_requests 
GROUP BY borough 
ORDER BY count DESC;

# View materialized view
SELECT * FROM mv_complaints_by_day_borough LIMIT 10;
```

---

## 🔄 Complete Workflow Example

### Step-by-Step ETL Process

```bash
# 1. Start fresh
make teardown  # Clean everything
make up        # Start PostgreSQL

# 2. Setup database
cargo run -- db init

# 3. Test with dry-run
cargo run -- run --input ./testdata/sample.csv --dry-run

# 4. Load data
cargo run -- run --input ./testdata/sample.csv

# 5. Verify data loaded
cargo run -- report last-run

# 6. Refresh analytics
cargo run -- db refresh-mv

# 7. Query results
docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux \
  -c "SELECT borough, COUNT(*) FROM service_requests GROUP BY borough;"
```

---

## 📁 Preparing Your Own CSV Data

UrbanFlux expects CSV files with these columns:

```csv
unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
```

**Column Requirements:**
- `unique_key`: Positive integer (required)
- `created_date`: Timestamp in format `YYYY-MM-DD HH:MM:SS` (required)
- `closed_date`: Timestamp or empty (optional)
- `complaint_type`: Non-empty text (required)
- `descriptor`: Text description (optional)
- `borough`: Must be one of: BRONX, BROOKLYN, MANHATTAN, QUEENS, STATEN ISLAND (optional)
- `latitude`: Float between 40.4 and 41.2 (optional)
- `longitude`: Float between -74.3 and -73.4 (optional)

**Example:**
```csv
unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
100001,2025-01-15 09:30:00,2025-01-15 11:00:00,Noise,Loud Music,MANHATTAN,40.7580,-73.9855
100002,2025-01-15 10:00:00,,Street Condition,Pothole,BROOKLYN,40.6782,-73.9442
```

---

## 🧪 Testing

### Run Unit Tests
```bash
cargo test
```

### Run with Verbose Output
```bash
cargo test -- --nocapture
```

### Run Specific Test
```bash
cargo test test_validate_borough
```

### Check Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

---

## 🔧 Configuration

Edit `.env` file to customize:

```bash
# Database connection
PGHOST=localhost
PGPORT=5432
PGUSER=urbanflux_user
PGPASSWORD=your_password
PGDATABASE=urbanflux

# ETL settings
ETL_CHUNK_SIZE=100000
ETL_MODE=full

# Logging
RUST_LOG=urbanflux=info,sqlx=warn
```

---

## 🐛 Troubleshooting

### "Failed to connect to PostgreSQL"
```bash
# Check if PostgreSQL is running
docker ps  # or
brew services list

# Check connection manually
psql -h localhost -U urbanflux_user -d urbanflux
```

### "File not found"
```bash
# Use absolute path
cargo run -- run --input $(pwd)/testdata/sample.csv

# Or relative from project root
cargo run -- run --input ./testdata/sample.csv
```

### "CSV parsing errors"
```bash
# Check CSV format
head -5 your_file.csv

# Validate headers
# Required: unique_key,created_date,closed_date,complaint_type,descriptor,borough,latitude,longitude
```

### "Port 5432 already in use"
```bash
# Stop existing PostgreSQL
brew services stop postgresql@16

# Or change port in docker-compose.yml
ports:
  - "5433:5432"  # Use 5433 on host
```

---

## 📈 Performance Tips

### For Large Datasets

```bash
# Increase chunk size for better throughput
cargo run -- run --input large_file.csv --chunk-size 500000

# Use RUST_LOG to reduce logging overhead
RUST_LOG=urbanflux=warn cargo run -- run --input large_file.csv

# Build in release mode for production
cargo build --release
./target/release/urbanflux run --input large_file.csv
```

### Database Tuning

For large loads, adjust PostgreSQL settings in `docker-compose.yml`:

```yaml
environment:
  POSTGRES_SHARED_BUFFERS: "256MB"
  POSTGRES_WORK_MEM: "16MB"
  POSTGRES_MAINTENANCE_WORK_MEM: "128MB"
```

---

## 🎯 What's Working Now

✅ **Fully Functional ETL Pipeline**
- Streaming CSV extraction with chunking
- Data validation and cleaning
- Deduplication by unique_key
- Bulk insert to PostgreSQL
- Error handling and logging

✅ **Database Operations**
- Schema initialization
- Index creation
- Materialized views
- View refresh

✅ **CLI Commands**
- `run` - Execute ETL pipeline
- `db init` - Initialize schema
- `db refresh-mv` - Refresh analytics
- `report last-run` - Show statistics

✅ **Data Quality**
- Borough validation (5 NYC boroughs)
- Coordinate validation (NYC bounding box)
- Date validation (closed >= created)
- Text cleaning and normalization

---

## 🚀 Next Steps

1. **Get NYC 311 Data**
   - Download from: https://data.cityofnewyork.us/Social-Services/311-Service-Requests-from-2010-to-Present/erm2-nwe9
   - Place in `testdata/` directory

2. **Run Full Pipeline**
   ```bash
   cargo run --release -- run --input testdata/311_data.csv --chunk-size 100000
   ```

3. **Build Analytics**
   - Query materialized views
   - Create dashboards
   - Generate reports

---

**🎉 Your ETL system is production-ready!**

All core functionality is implemented and tested. You can now process real NYC 311 data at scale!
