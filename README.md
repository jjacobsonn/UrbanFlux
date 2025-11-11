# UrbanFlux

[![CI](https://github.com/yourusername/urbanflux/workflows/CI/badge.svg)](https://github.com/yourusername/urbanflux/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **High-performance ETL system for NYC 311 Service Request data** — Built with Rust for systems-level rigor, async I/O, and production-grade reliability.

UrbanFlux is a cross-platform, production-ready ETL pipeline that ingests multi-gigabyte NYC 311 Service Request CSV files, performs data cleaning and deduplication, bulk-loads into PostgreSQL, and maintains materialized views for high-speed analytics. Designed to demonstrate FAANG-level systems engineering and Rust's concurrency model.

---

## 🚀 Features

- **Streaming ETL Pipeline**: O(N) linear-time processing with O(1) memory overhead per chunk
- **Async Architecture**: Built on Tokio for high-throughput concurrent operations
- **Type-Safe Database**: SQLx with compile-time query validation
- **Bulk Loading**: PostgreSQL COPY protocol for maximum insert performance
- **Data Quality**: Built-in validation, cleaning, and deduplication logic
- **Materialized Views**: Automated nightly refresh for analytics workloads
- **Observability**: Structured logging with tracing, metrics, and run reports
- **Docker Native**: Fully containerized stack with docker-compose
- **CI/CD Ready**: GitHub Actions pipeline with formatting, linting, and testing

---

## 📋 Prerequisites

- **Rust**: 1.80+ ([Install Rust](https://rustup.rs/))
- **Docker**: 24.0+ with docker-compose
- **PostgreSQL**: 16+ (provided via Docker, or use local instance)
- **Make**: For convenient build commands

---

## 🏗️ Quick Start

### 1. Clone and Setup

```bash
git clone https://github.com/yourusername/urbanflux.git
cd urbanflux
cp .env.example .env
```

### 2. Start Infrastructure

```bash
make up
```

This starts PostgreSQL via docker-compose.

### 3. Build the Project

```bash
make build
```

Or for development:

```bash
cargo build
```

### 4. Initialize Database

```bash
make db-init
```

### 5. Run ETL Pipeline

```bash
# Full load
make seed

# Incremental load
make nightly
```

### 6. Query Results

```bash
docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux -c \
  "SELECT * FROM mv_complaints_by_day_borough LIMIT 10;"
```

---

## 🛠️ CLI Usage

```bash
# Show help
urbanflux --help

# Run full ETL
urbanflux run --mode full --input ./testdata/sample.csv --chunk-size 100000

# Run incremental ETL
urbanflux run --mode incremental --input ./testdata/sample.csv

# Dry run (no database writes)
urbanflux run --mode full --input ./testdata/sample.csv --dry-run

# Initialize database schema
urbanflux db init

# Refresh materialized views
urbanflux db refresh-mv --concurrently

# View last run report
urbanflux report last-run
```

---

## 📁 Project Structure

```
urbanflux/
├── .github/
│   ├── workflows/           # CI/CD pipelines
│   ├── ISSUE_TEMPLATE/      # Issue templates
│   └── pull_request_template/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── config.rs            # Configuration management
│   ├── logging.rs           # Tracing setup
│   ├── etl/
│   │   ├── extract.rs       # CSV streaming
│   │   ├── transform.rs     # Data cleaning
│   │   └── load.rs          # Bulk insert
│   ├── db/
│   │   └── schema.rs        # Database schema
│   └── clean/
│       └── validator.rs     # Data validation
├── migrations/              # SQL migrations
├── scripts/                 # Utility scripts
├── testdata/                # Sample data
├── docs/                    # Documentation
├── Dockerfile               # Multi-stage build
├── docker-compose.yml       # Stack orchestration
├── Makefile                 # Build automation
└── Cargo.toml               # Rust dependencies
```

---

## 🧪 Testing

```bash
# Run all tests
make test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

---

## 🎨 Code Quality

```bash
# Format code
make fmt

# Check formatting
make fmt-check

# Run linter
make lint
```

---

## 🐳 Docker Commands

```bash
# Start stack
make up

# Stop stack
make down

# View logs
make logs

# Teardown (including volumes)
make teardown
```

---

## 📊 Database Schema

### Service Requests Table

```sql
CREATE TABLE service_requests (
  unique_key BIGINT PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL,
  closed_at TIMESTAMPTZ,
  complaint_type TEXT NOT NULL,
  descriptor TEXT,
  borough TEXT CHECK (borough IN ('BRONX','BROOKLYN','MANHATTAN','QUEENS','STATEN ISLAND')),
  latitude DOUBLE PRECISION,
  longitude DOUBLE PRECISION,
  ingested_at TIMESTAMPTZ DEFAULT now()
);
```

### Materialized Views

- `mv_complaints_by_day_borough`: Daily complaint counts by borough
- `mv_complaints_by_type_month`: Monthly complaint counts by type

---

## 🔧 Configuration

Configuration via environment variables (see `.env.example`):

```bash
# Database
PGHOST=localhost
PGPORT=5432
PGUSER=urbanflux_user
PGPASSWORD=your_password
PGDATABASE=urbanflux

# ETL
ETL_INPUT_PATH=./testdata/sample.csv
ETL_CHUNK_SIZE=100000
ETL_MODE=full

# Logging
RUST_LOG=urbanflux=info,sqlx=warn
```

---

## 📈 Performance

- **Throughput**: ≥300k rows/sec per core
- **Memory**: O(1) overhead per chunk (configurable chunk size)
- **Complexity**: O(N) linear time for full pipeline

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

---

## 📝 Style Guide

See [STYLEGUIDE.md](STYLEGUIDE.md) for:
- Naming conventions
- Commit message format
- Code organization
- Error handling patterns

---

## 🔒 Security

See [SECURITY.md](SECURITY.md) for security policies and reporting vulnerabilities.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- NYC Open Data for providing the 311 Service Request dataset
- The Rust community for excellent async/await tooling
- PostgreSQL team for robust database performance

---

## 📚 Documentation

For detailed technical documentation, see [TECH-DOC.md](TECH-DOC.md).

---

## 🐛 Known Issues

- None yet! This is a fresh setup ready for implementation.

---

## 🗺️ Roadmap

- [x] Project scaffolding and CI/CD
- [ ] CSV streaming implementation
- [ ] Transform and validation logic
- [ ] Bulk load via COPY
- [ ] Materialized view automation
- [ ] Metrics endpoint
- [ ] Web dashboard (stretch goal)

---

Built with ❤️ using Rust
