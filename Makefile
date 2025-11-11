.PHONY: help build test lint fmt clean up down seed nightly logs teardown

# Default target
help:
	@echo "UrbanFlux - Makefile targets:"
	@echo "  build       - Build the Rust project"
	@echo "  test        - Run all tests"
	@echo "  lint        - Run clippy linter"
	@echo "  fmt         - Format code with rustfmt"
	@echo "  clean       - Clean build artifacts"
	@echo "  up          - Start Docker Compose stack"
	@echo "  down        - Stop Docker Compose stack"
	@echo "  seed        - Run full ETL load"
	@echo "  nightly     - Run incremental ETL + refresh MVs"
	@echo "  logs        - Show container logs"
	@echo "  teardown    - Stop and remove all containers and volumes"
	@echo "  db-init     - Initialize database schema"
	@echo "  db-refresh  - Refresh materialized views"

# Build targets
build:
	@echo "Building UrbanFlux..."
	cargo build --release

build-dev:
	@echo "Building UrbanFlux (dev)..."
	cargo build

# Testing
test:
	@echo "Running tests..."
	cargo test --all

test-verbose:
	@echo "Running tests (verbose)..."
	cargo test --all -- --nocapture

# Code quality
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

fmt:
	@echo "Formatting code..."
	cargo fmt

fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

# Cleanup
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/

# Docker operations
up:
	@echo "Starting Docker Compose stack..."
	docker compose up -d

down:
	@echo "Stopping Docker Compose stack..."
	docker compose down

logs:
	@echo "Showing container logs..."
	docker compose logs -f

teardown:
	@echo "Tearing down Docker Compose stack..."
	docker compose down -v
	rm -rf pgdata/

# ETL operations
seed:
	@echo "Running full ETL load..."
	docker compose run --rm etl run --mode full --input /app/testdata/sample.csv

nightly:
	@echo "Running incremental ETL..."
	docker compose run --rm etl run --mode incremental --input /app/testdata/sample.csv

# Database operations
db-init:
	@echo "Initializing database schema..."
	docker compose run --rm etl db init

db-refresh:
	@echo "Refreshing materialized views..."
	docker compose run --rm etl db refresh-mv --concurrently

# Local development
dev-run:
	@echo "Running locally (requires local Postgres)..."
	cargo run -- --help

dev-watch:
	@echo "Running with cargo watch..."
	cargo watch -x run
