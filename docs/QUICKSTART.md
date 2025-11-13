# 🚀 UrbanFlux Quick Start Guide

## For First-Time Setup from GitHub

### 1. Clone & Setup (One Command!)

```bash
git clone https://github.com/jjacobsonn/UrbanFlux.git
cd UrbanFlux
./setup.sh install
```

**What happens:**
- ✅ Checks Docker, Rust, and other dependencies
- ✅ Creates `.env` file with configuration
- ✅ Builds Rust project (may take 5-10 minutes first time)
- ✅ Starts PostgreSQL in Docker container
- ✅ Runs database migrations
- ✅ Loads sample test data (10 records)
- ✅ Refreshes materialized views
- ✅ Runs health check

**Time:** ~5-10 minutes (mostly Rust compilation)

---

## 2. Interactive Menu (Easiest!)

If you prefer a menu-driven interface:

```bash
./setup.sh menu
```

Navigate with numbers 1-13. Perfect for:
- Exploring the system
- Running individual operations
- Learning what's available

---

## 3. Manual Commands

For command-line power users:

```bash
# Start everything
./setup.sh start

# Run migrations
./setup.sh migrate

# Load data
./setup.sh load

# Check health
./setup.sh health

# View data
./setup.sh query

# Stop everything
./setup.sh stop
```

---

## 4. ETL Pipeline Usage

Once installed, process your own data:

```bash
# Full ETL run
cargo run --release -- run --mode full --input your_data.csv

# With custom chunk size
cargo run --release -- run --mode full --input your_data.csv --chunk-size 50000

# Dry run (validate only, don't insert)
cargo run --release -- run --mode full --input your_data.csv --dry-run
```

---

## 5. View Results

```bash
# Quick sample query via script
./setup.sh query

# Last ETL run statistics
cargo run -- report last-run

# Direct PostgreSQL access
docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux

# Then run SQL:
SELECT * FROM mv_complaints_by_day_borough LIMIT 10;
SELECT * FROM mv_complaints_by_type_month LIMIT 10;
```

---

## 6. Troubleshooting

### "Docker not found"
```bash
# Install Docker:
# Mac: https://docs.docker.com/desktop/install/mac-install/
# Ubuntu: sudo apt-get install docker.io docker-compose
# Windows: https://docs.docker.com/desktop/install/windows-install/
```

### "Rust not found"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "Build failed" or other issues
```bash
# Complete reset and retry
./setup.sh reset
./setup.sh install
```

### "PostgreSQL not responding"
```bash
# Check container
docker ps

# View logs
docker compose logs postgres

# Restart
./setup.sh stop
./setup.sh start
```

---

## 7. Reset Everything

Need a fresh start?

```bash
./setup.sh reset
```

This removes:
- Docker containers and volumes
- Build artifacts
- `.env` file

Then reinstall:
```bash
./setup.sh install
```

---

## 8. Daily Workflow

```bash
# Morning: Start services
./setup.sh start

# Work: Run ETL, develop features
cargo run -- run --mode full --input data.csv

# Check results
./setup.sh query

# Evening: Stop services
./setup.sh stop
```

---

## 9. Development Workflow

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build for production
cargo build --release

# Or use script
./setup.sh test
```

---

## 10. Before Pushing to GitHub

```bash
# Ensure everything works
./setup.sh test

# Clean up
cargo fmt
cargo clippy

# Test fresh install
./setup.sh reset
./setup.sh install
```

---

## Common Operations Reference

| Task | Command |
|------|---------|
| First setup | `./setup.sh install` |
| Interactive menu | `./setup.sh menu` |
| Start services | `./setup.sh start` |
| Run ETL | `cargo run -- run --mode full --input data.csv` |
| Check health | `./setup.sh health` |
| View data | `./setup.sh query` |
| Stop services | `./setup.sh stop` |
| Run tests | `./setup.sh test` |
| Complete reset | `./setup.sh reset` |
| Get help | `./setup.sh help` |

---

## Expected Output (Successful Installation)

```
========================================
🚀 Full Installation: UrbanFlux
========================================

========================================
Checking System Dependencies
========================================

✓ Docker is installed (24.0.7)
✓ Docker Compose is installed
✓ Rust is installed (1.80.0)
✓ All required dependencies are installed

========================================
Setting Up Environment
========================================

✓ Created default .env file
✓ Environment variables configured

========================================
Building Rust Project
========================================

ℹ Running cargo build...
   Compiling urbanflux v0.1.0
    Finished release [optimized] target(s) in 3m 42s
✓ Project built successfully

========================================
Starting Docker Containers
========================================

✓ PostgreSQL container started
✓ PostgreSQL is ready

========================================
Running Database Migrations
========================================

✓ Migrations completed successfully

========================================
Loading Test Data
========================================

✓ Test data loaded successfully
ℹ Records in database: 10

========================================
Refreshing Materialized Views
========================================

✓ Materialized views refreshed

========================================
✨ Installation Complete!
========================================

✓ UrbanFlux is ready to use

Quick commands:
  ./setup.sh menu      - Show interactive menu
  ./setup.sh health    - Check system health
  ./setup.sh query     - View sample data
  ./setup.sh stop      - Stop services
  ./setup.sh reset     - Complete reset
```

---

## Next Steps

1. **Explore the data**: `./setup.sh query`
2. **Process your own CSV**: `cargo run -- run --mode full --input your_data.csv`
3. **Read the docs**: Check `README.md` and `TECH-DOC.md`
4. **Run tests**: `./setup.sh test`
5. **Customize**: Edit `.env` file for your needs

---

## Getting Help

- **In-app help**: `./setup.sh help` or `./setup.sh menu`
- **CLI help**: `cargo run -- --help`
- **Issues**: https://github.com/jjacobsonn/UrbanFlux/issues
- **Documentation**: See `README.md` for detailed docs

---

**Happy data processing! 🎉**
