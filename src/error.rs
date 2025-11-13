// Custom error types for the application
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtlError {
    #[error("CSV parsing error: {0}")]
    CsvParse(String),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Invalid data: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Date/time parsing error: {0}")]
    DateTimeParse(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Watermark error: {0}")]
    Watermark(String),
}

pub type Result<T> = std::result::Result<T, EtlError>;
