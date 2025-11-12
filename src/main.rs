// Module declarations
pub mod clean;
pub mod config;
pub mod db;
pub mod etl;
pub mod logging;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser, Debug)]
#[command(
    name = "urbanflux",
    version,
    author,
    about = "High-performance ETL system for NYC 311 Service Request data",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run ETL pipeline
    Run {
        /// Mode: full or incremental
        #[arg(short, long, default_value = "full")]
        mode: String,

        /// Input CSV path or URL
        #[arg(short, long)]
        input: String,

        /// Chunk size for batch processing
        #[arg(short, long, default_value = "100000")]
        chunk_size: usize,

        /// Dry run without database writes
        #[arg(long, default_value = "false")]
        dry_run: bool,
    },
    /// Database operations
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    /// Generate report from last run
    Report {
        #[command(subcommand)]
        command: ReportCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DbCommands {
    /// Initialize database schema and tables
    Init,
    /// Refresh materialized views
    RefreshMv {
        /// Use CONCURRENTLY for non-blocking refresh
        #[arg(long, default_value = "false")]
        concurrently: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ReportCommands {
    /// Show last run summary
    LastRun,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::init()?;

    info!("UrbanFlux ETL System starting...");

    // Load configuration
    let config = config::Config::from_env()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            mode,
            input,
            chunk_size,
            dry_run,
        } => {
            info!(
                mode = %mode,
                input = %input,
                chunk_size = chunk_size,
                dry_run = dry_run,
                "Running ETL pipeline"
            );

            if dry_run {
                println!("🔍 DRY RUN MODE - No database writes will occur\n");
            }

            // Extract
            println!("📥 Extracting data from CSV...");
            let extractor = etl::Extractor::new(chunk_size);
            let chunks = extractor.extract(&input).await?;

            let total_extracted: usize = chunks.iter().map(|c| c.len()).sum();
            println!("✅ Extracted {} records in {} chunks", total_extracted, chunks.len());

            // Transform and Load
            let mut total_loaded = 0u64;
            let mut total_rejected = 0usize;

            let transformer = etl::Transformer::new();

            // Only connect to DB if not in dry-run mode
            let db = if !dry_run {
                Some(db::Database::connect(&config.database_url()).await?)
            } else {
                None
            };

            for (i, chunk) in chunks.into_iter().enumerate() {
                println!("🔄 Processing chunk {}/...", i + 1);
                
                let initial_count = chunk.len();
                let clean_records = transformer.transform(chunk)?;
                let rejected = initial_count - clean_records.len();
                total_rejected += rejected;

                if let Some(ref database) = db {
                    let loader = etl::Loader::new(database.clone());
                    let inserted = loader.load(clean_records).await?;
                    total_loaded += inserted;
                }
            }

            println!("\n📊 ETL Summary:");
            println!("  Total extracted: {}", total_extracted);
            println!("  Total rejected:  {}", total_rejected);
            if let Some(ref database) = db {
                println!("  Total loaded:    {}", total_loaded);
                
                let count = database.get_record_count().await?;
                println!("  Records in DB:   {}", count);
            } else {
                println!("  Would load:      {}", total_extracted - total_rejected);
            }

            println!("\n✨ ETL pipeline completed successfully!");
            Ok(())
        }
        Commands::Db { command } => match command {
            DbCommands::Init => {
                info!("Initializing database schema");
                println!("🔧 Initializing database schema...");

                let db = db::Database::connect(&config.database_url()).await?;
                db.initialize_schema().await?;

                println!("✅ Database schema initialized successfully!");
                Ok(())
            }
            DbCommands::RefreshMv { concurrently } => {
                info!(concurrently = concurrently, "Refreshing materialized views");
                println!("🔄 Refreshing materialized views...");

                let db = db::Database::connect(&config.database_url()).await?;
                db.refresh_materialized_views(concurrently).await?;

                println!("✅ Materialized views refreshed successfully!");
                Ok(())
            }
        },
        Commands::Report { command } => match command {
            ReportCommands::LastRun => {
                info!("Generating last run report");
                println!("📊 Last Run Report");
                println!("─────────────────────");

                let db = db::Database::connect(&config.database_url()).await?;
                let count = db.get_record_count().await?;

                println!("Total records in database: {}", count);
                println!("\nNote: Detailed run reports will be implemented in future phase");
                Ok(())
            }
        },
    }
}
