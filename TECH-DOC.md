# UrbanFlux – Rust ETL System Technical Spec (AI-Agent Ready)

> **Purpose:** This Markdown instructs an AI coding agent to build **UrbanFlux**, a cross-platform, production-grade ETL system in **Rust**. It ingests multi-GB NYC 311 Service Request CSVs, cleans and deduplicates them, bulk-loads into PostgreSQL, and maintains **materialized views** for high-speed analytics with nightly automation. Emphasis: **systems-level rigor**, **Big-O scalability**, **correctness**, **safety**, and **FAANG-level reliability engineering.**

---

## 0) Executive Summary
- **Problem:** NYC 311 datasets (tens of millions of rows) need scalable ingestion, cleaning, and analytics.
- **Solution:** Streamed ETL in Rust using async I/O (**Tokio**), **O(N)** linear time, **O(1)** per-chunk memory, bulk COPY to PostgreSQL, and nightly **REFRESH MATERIALIZED VIEW**.
- **Deliverables:** Dockerized Rust stack (Linux/macOS/Windows), CLI ETL binary, schema + materialized views, metrics, tests, CI/CD pipeline, and demo script.
- **Outcome:** A reproducible, high-throughput system demonstrating Rust’s concurrency, type safety, and low-level performance for interviews.

---

## 1) Scope & Non-Goals
**In Scope**
- Single-node, async batch ETL (initial load + nightly incremental updates).
- Rust async architecture with **Tokio**, **SQLx**, and **csv_async**.
- PostgreSQL schema, indexes, constraints, and views.
- Structured logging (via **tracing**), metrics, health checks, retries, idempotence.
- Unit/integration/property tests and CI pipeline.

**Non-Goals (v1)**
- Distributed compute (Spark/Kafka) or streaming SLAs.
- Web dashboard or visualization.

---

## 2) Language & Stack
- **Language:** Rust 1.80+
- **Async runtime:** Tokio
- **DB Layer:** SQLx (async Postgres driver with compile-time query validation)
- **CSV Parser:** csv_async crate
- **Logging/Tracing:** tracing + tracing-subscriber
- **CLI Framework:** clap
- **Serialization:** serde, serde_json
- **Testing:** proptest, assert_cmd, test-containers
- **Containerization:** Docker + docker-compose
- **CI/CD:** GitHub Actions (cargo fmt, clippy, test, docker build)

**Why Rust?**
- **Memory-safe** concurrency and deterministic performance.
- Zero-copy parsing for streaming CSV.
- Demonstrates systems-level proficiency and performance engineering — key for FAANG interviews.

---

## 3) Architecture Overview
**Stages**
1. **Extract:** Stream CSV → typed structs (no full-file load).
2. **Transform:** Normalize, clean, deduplicate, produce valid rows.
3. **Load:** Bulk ingest via `COPY FROM STDIN` (SQLx raw connection).
4. **Optimize:** Run `ANALYZE`, build indexes, refresh **materialized views**.
5. **Schedule:** Automated nightly runs; idempotent, logged, observable.

**Performance Targets**
- O(N) per batch, O(1) memory overhead.
- Throughput: ≥300k rows/sec per core.

---

## 4) Data Contracts
**Input CSV:**
- `unique_key` (bigint)
- `created_date`, `closed_date` (timestamps)
- `complaint_type`, `descriptor` (text)
- `borough`, `latitude`, `longitude`

**Output Schema:** `public.service_requests`
```sql
CREATE TABLE IF NOT EXISTS service_requests (
  unique_key BIGINT PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL,
  closed_at TIMESTAMPTZ NULL,
  complaint_type TEXT NOT NULL,
  descriptor TEXT,
  borough TEXT CHECK (borough IN ('BRONX','BROOKLYN','MANHATTAN','QUEENS','STATEN ISLAND')),
  latitude DOUBLE PRECISION,
  longitude DOUBLE PRECISION,
  ingested_at TIMESTAMPTZ DEFAULT now()
);
```

**Materialized Views:**
- `mv_complaints_by_day_borough`
- `mv_complaints_by_type_month`

Indexes: PK + B-Tree on `(created_at)`, `(borough)`, and MV columns.

---

## 5) ETL Semantics & Algorithms
### Extraction
- Use **csv_async::AsyncReader** over `tokio::fs::File`.
- Stream in configurable `chunk_size` (default: 100_000 rows).
- Parse timestamps using `chrono` or `time` crate.
- Complexity: O(N) time, O(1) extra memory.

### Transformation
- Clean categorical fields; normalize timestamps to UTC.
- Drop malformed or incomplete rows; quarantine to `/bad_rows/YYYYMMDD.csv`.
- Dedup via in-run **HashSet** of `unique_key` + DB PK enforcement.

### Loading
- Use **SQLx::copy_in_raw** or direct COPY command via `sqlx::Executor`.
- Commit per chunk, transactional.
- Incremental mode queries new records using watermark control table.

### Optimization
- Run `ANALYZE` after inserts.
- Refresh MVs (`REFRESH MATERIALIZED VIEW CONCURRENTLY`).
- Complexity: O(N) overall.

---

## 6) Reliability & Idempotence
- **Watermarks:** track last loaded `created_at` and `unique_key`.
- **Retries:** exponential backoff with `tokio_retry`.
- **Atomicity:** COPY transactions; rollbacks on failure.
- **Crash Safety:** Resumable incremental load via watermark table.

---

## 7) Observability
**Logging:** `tracing` JSON output (stage, chunk, row counts, duration).
**Metrics:** optional `/metrics` endpoint using `axum` + `prometheus` exporter.
**Reports:** per-run JSON written to `/runs/YYYYMMDD.json`.

---

## 8) Security & Config
- Environment variables loaded via `dotenvy`.
- No secrets in logs.
- Use `ingest_role` DB user (INSERT only) + `report_role` (read + refresh).

**ENV Variables**
```
PGHOST, PGPORT, PGUSER, PGPASSWORD, PGDATABASE
ETL_INPUT_PATH, ETL_CHUNK_SIZE, ETL_MODE (full|incremental)
```

---

## 9) Packaging & Cross-Platform
- Docker + Compose stack: `postgres`, `urbanflux-etl`.
- Works cross-OS.

**Make Targets**
```
make up          # start stack
docker compose up -d
make seed        # full load
make nightly     # incremental + refresh MVs
make teardown    # stop stack
```

---

## 10) Database Objects
Includes service_requests, MVs, and control table `etl_watermarks`.

---

## 11) Scheduler
- Nightly cron or `tokio_cron_scheduler` inside container.
- Idempotent runs with resumable state.

---

## 12) CLI Contracts
```
urbanflux run --mode [full|incremental] --input <path|URL> --chunk-size <int> --dry-run
urbanflux db init
urbanflux db refresh-mv [--concurrently]
urbanflux report last-run
```

Exit codes: 0 = success, 10/20/30/40 = stage failures.

---

## 13) Testing Strategy
- **Unit:** parsers, cleaners, dedupe logic.
- **Integration:** with Dockerized Postgres via testcontainers.
- **Property:** dedupe idempotence (proptest).
- **Perf:** synthetic 10M-row stream; assert ≥250k rows/sec.

---

## 14) CI/CD (GitHub Actions)
```yaml
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with: { toolchain: stable, override: true }
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test --all --release
      - run: docker build -t urbanflux .
```

---

## 15) Demo Playbook
1. `make up`
2. `make seed`
3. `urbanflux db refresh-mv`
4. Query:
```sql
SELECT * FROM mv_complaints_by_day_borough LIMIT 5;
```
5. Show metrics/logs in terminal.

---

## 16) Rust Implementation Notes
- Async entrypoint with `#[tokio::main]`.
- Modular layout:
```
/cmd/urbanflux/main.rs
/src/etl/extract.rs
/src/etl/transform.rs
/src/etl/load.rs
/src/db/schema.rs
/src/config.rs
/src/logging.rs
/src/tests/
```
- Error handling: `anyhow::Result<>` + `thiserror` for domain errors.
- Structured logs via `tracing_subscriber::fmt().json()`.
- Use Rust’s ownership to ensure clean memory and safe concurrency.

---

## 17) Coding Conventions
- snake_case for files and DB columns.
- PascalCase for structs/enums.
- CI enforced formatting (cargo fmt, clippy).
- Exhaustive error handling; all Results unwrapped safely.

---

## 18) Acceptance Criteria
- Runs on all OS via Docker.
- Full load completes without OOM.
- Incremental updates idempotent.
- MVs refreshed, queries <200ms.
- Tests + CI green.

---

## 19) Stretch Goals
- Partitioned Postgres tables by year.
- S3/HTTP ingestion with `reqwest`.
- Web UI via Axum dashboard.
- Grafana dashboard integration.

---

## 20) Prompts for AI Agent
Implement per spec:
1. Rust CLI with clap.
2. Async stream CSV reader.
3. Cleaning, dedupe (HashSet), and watermarks.
4. COPY loader via SQLx.
5. MV refresh logic.
6. Logging + metrics.
7. Docker + Makefile.
8. Unit + integration tests.

**Do NOT:** load full CSV to memory or log credentials.

---

## 21) Data Quality Rules
- Borough whitelist.
- created_at required; closed_at ≥ created_at.
- Lat [40,41.2]; Lon [-74.3,-73.4].
- Trim, normalize categorical fields.

---

## 22) Run Report JSON
```json
{
  "run_id": "2025-11-11T02:00:00Z",
  "mode": "incremental",
  "input": "https://data.city.gov/311.csv",
  "rows": {"in": 84512, "clean": 84290, "dupe": 150, "bad": 72},
  "durations_ms": {"extract": 12000, "transform": 18000, "load": 9000, "refresh_mv": 45000},
  "watermarks": {"last_created_at": "2025-11-10T23:59:59Z", "last_unique_key": 1234567890},
  "status": "success"
}
```

---

**End of Spec — UrbanFlux is a Rust-based ETL system demonstrating concurrency, performance, and reliability suitable for FAANG interviews.**
