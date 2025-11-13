// UrbanFlux CLI entrypoint
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use urbanflux::database::{Database, ServiceRequestRepository, WatermarkRepository};
use urbanflux::domain::{EtlMode, EtlStats};
use urbanflux::extract::CsvRecordStream;
use urbanflux::transform::TransformProcessor;
use urbanflux::{Config, Result};

#[derive(Parser)]
#[command(
    name = "urbanflux",
    version,
    author,
    about = "Production-grade ETL system for NYC 311 Service Request data"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run ETL pipeline
    Run {
        /// ETL mode: full or incremental
        #[arg(short, long, default_value = "full")]
        mode: String,

        /// Input CSV file path
        #[arg(short, long)]
        input: PathBuf,

        /// Chunk size for batch processing
        #[arg(short, long, default_value = "100000")]
        chunk_size: usize,

        /// Dry run (validate without database writes)
        #[arg(long)]
        dry_run: bool,
    },
    /// Database operations
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    /// Report operations
    Report {
        #[command(subcommand)]
        command: ReportCommands,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Run database migrations
    Migrate,
    /// Refresh materialized views
    RefreshMv {
        /// Use CONCURRENTLY option
        #[arg(long)]
        concurrently: bool,
    },
    /// Check database health
    Health,
}

#[derive(Subcommand)]
enum ReportCommands {
    /// Show last ETL run statistics
    LastRun,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::from_env()?;

    // Initialize logging
    urbanflux::logging::init(&config.logging)?;

    info!("UrbanFlux ETL System starting");

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            mode,
            input,
            chunk_size,
            dry_run,
        } => {
            run_etl(config, mode, input, chunk_size, dry_run).await?;
        }
        Commands::Db { command } => match command {
            DbCommands::Migrate => {
                info!("Running database migrations");
                let db = Database::connect(&config.database).await?;
                db.migrate().await?;
                println!("✅ Migrations completed successfully");
            }
            DbCommands::RefreshMv { concurrently } => {
                info!("Refreshing materialized views");
                let db = Database::connect(&config.database).await?;
                db.refresh_materialized_views(concurrently).await?;
                println!("✅ Materialized views refreshed");
            }
            DbCommands::Health => {
                info!("Checking database health");
                let db = Database::connect(&config.database).await?;
                db.health_check().await?;
                println!("✅ Database connection healthy");
            }
        },
        Commands::Report { command } => match command {
            ReportCommands::LastRun => {
                let db = Database::connect(&config.database).await?;
                let repo = ServiceRequestRepository::new(db);
                let count = repo.count().await?;
                println!("📊 Total records in database: {}", count);
            }
        },
    }

    Ok(())
}

async fn run_etl(
    config: Config,
    mode_str: String,
    input: PathBuf,
    chunk_size: usize,
    dry_run: bool,
) -> Result<()> {
    let mode: EtlMode = mode_str.parse().map_err(|e| urbanflux::EtlError::Config(e))?;

    info!(
        mode = ?mode,
        input = ?input,
        chunk_size = chunk_size,
        dry_run = dry_run,
        "Starting ETL pipeline"
    );

    if dry_run {
        println!("🔍 DRY RUN MODE - No database writes\n");
    }

    // Connect to database (unless dry run)
    let db = if !dry_run {
        Some(Database::connect(&config.database).await?)
    } else {
        None
    };

    // Initialize components
    let stream = CsvRecordStream::new(chunk_size);
    let mut transformer = TransformProcessor::new();

    // Start ETL run watermark
    let run_id = if let Some(ref database) = db {
        let watermark_repo = WatermarkRepository::new(database.clone());
        Some(watermark_repo.start_run(mode).await?)
    } else {
        None
    };

    info!(run_id = ?run_id, "ETL run started");

    // Process chunks
    let chunks = stream.stream_chunks(&input).await?;
    let mut total_stats = EtlStats::new();

    for (chunk_num, (records, chunk_stats)) in chunks.into_iter().enumerate() {
        let chunk_num = chunk_num + 1;
        info!(chunk = chunk_num, records = records.len(), "Processing chunk");

        // Transform
        let (clean_records, mut chunk_stats) = transformer.process(records, chunk_stats);

        // Load
        if let Some(ref database) = db {
            let repo = ServiceRequestRepository::new(database.clone());
            let inserted = repo.bulk_insert(&clean_records).await?;
            chunk_stats.rows_inserted = inserted;
            info!(chunk = chunk_num, inserted = inserted, "Chunk loaded");
        }

        total_stats.merge(&chunk_stats);
    }

    // Complete run
    if let (Some(database), Some(run_id)) = (db, run_id) {
        let watermark_repo = WatermarkRepository::new(database.clone());
        watermark_repo.complete_run(run_id, &total_stats).await?;
    }

    // Print summary
    println!("\n📊 ETL Summary:");
    println!("  Rows read:       {}", total_stats.rows_read);
    println!("  Rows parsed:     {}", total_stats.rows_parsed);
    println!("  Rows validated:  {}", total_stats.rows_validated);
    println!("  Rows inserted:   {}", total_stats.rows_inserted);
    println!("  Duplicates:      {}", total_stats.rows_duplicated);
    println!("  Rejected:        {}", total_stats.rows_rejected);
    println!("  Parse errors:    {}", total_stats.parse_errors);
    println!("\n✨ ETL pipeline completed successfully!");

    Ok(())
}
