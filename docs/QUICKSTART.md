# Quick Start Guide

This guide will help you get UrbanFlux up and running in minutes.

## Prerequisites Checklist

- [ ] Rust 1.80+ installed (`rustc --version`)
- [ ] Docker Desktop running (`docker --version`)
- [ ] Git configured
- [ ] 4GB+ RAM available

## 5-Minute Setup

### Step 1: Clone and Enter

```bash
git clone https://github.com/jjacobsonn/UrbanFlux.git
cd UrbanFlux
```

### Step 2: Configure Environment

```bash
cp .env.example .env
# Edit .env if needed (defaults work for local development)
```

### Step 3: Start Services

```bash
make up
```

This starts PostgreSQL in Docker. Wait ~10 seconds for it to initialize.

### Step 4: Verify Build

```bash
cargo build
cargo test
cargo run -- --help
```

### Step 5: Initialize Database

```bash
make db-init
# Or manually:
# docker exec -it urbanflux-postgres psql -U urbanflux_user -d urbanflux -f /docker-entrypoint-initdb.d/001_init_schema.sql
```

## Verify Installation

```bash
# Check cargo
cargo --version

# Check Docker
docker ps

# Test CLI
cargo run -- --help

# Should show:
# Usage: urbanflux <COMMAND>
# Commands:
#   run     Run ETL pipeline
#   db      Database operations
#   report  Generate report from last run
```

## Next Steps

1. **Read the full README**: `../README.md`
2. **Explore the code**: Start with `src/main.rs`
3. **Run tests**: `cargo test`
4. **Check style guide**: `STYLEGUIDE.md`
5. **Contribute**: See `CONTRIBUTING.md`

## Common Commands

```bash
# Development
make build          # Build project
make test           # Run tests
make fmt            # Format code
make lint           # Run linter

# Docker
make up             # Start stack
make down           # Stop stack
make logs           # View logs
make teardown       # Full cleanup

# Database
make db-init        # Initialize schema
make db-refresh     # Refresh materialized views
```

## Troubleshooting

**"Port 5432 already in use"**
```bash
# Stop existing PostgreSQL
brew services stop postgresql
# Or change port in docker-compose.yml
```

**"Cannot connect to Docker"**
```bash
# Ensure Docker Desktop is running
open -a Docker
```

**"Cargo build fails"**
```bash
# Update Rust
rustup update
# Clean and rebuild
cargo clean && cargo build
```

## Success Criteria

✅ `cargo build` succeeds  
✅ `cargo test` passes  
✅ `docker ps` shows running postgres  
✅ `cargo run -- --help` displays CLI help  

---

You're ready to use the ETL pipeline.
