-- Initial schema for UrbanFlux ETL system
-- This file is automatically executed when the PostgreSQL container starts

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create service_requests table
CREATE TABLE IF NOT EXISTS service_requests (
    unique_key BIGINT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    closed_at TIMESTAMPTZ,
    complaint_type TEXT NOT NULL,
    descriptor TEXT,
    borough TEXT CHECK (borough IN ('BRONX', 'BROOKLYN', 'MANHATTAN', 'QUEENS', 'STATEN ISLAND')),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    ingested_at TIMESTAMPTZ DEFAULT now()
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_service_requests_created_at 
    ON service_requests(created_at);
CREATE INDEX IF NOT EXISTS idx_service_requests_borough 
    ON service_requests(borough);
CREATE INDEX IF NOT EXISTS idx_service_requests_complaint_type 
    ON service_requests(complaint_type);

-- Create ETL watermarks table for incremental loads
CREATE TABLE IF NOT EXISTS etl_watermarks (
    id SERIAL PRIMARY KEY,
    run_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    last_created_at TIMESTAMPTZ,
    last_unique_key BIGINT,
    run_mode TEXT NOT NULL CHECK (run_mode IN ('full', 'incremental')),
    rows_processed BIGINT NOT NULL DEFAULT 0,
    rows_inserted BIGINT NOT NULL DEFAULT 0,
    rows_skipped BIGINT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'completed', 'failed'))
);

-- Create materialized view for complaints by day and borough
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_complaints_by_day_borough AS
SELECT 
    DATE(created_at) as complaint_date,
    borough,
    COUNT(*) as complaint_count
FROM service_requests
WHERE borough IS NOT NULL
GROUP BY DATE(created_at), borough
ORDER BY complaint_date DESC, borough;

-- Create index on materialized view
CREATE INDEX IF NOT EXISTS idx_mv_complaints_date_borough 
    ON mv_complaints_by_day_borough(complaint_date, borough);

-- Create materialized view for complaints by type and month
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_complaints_by_type_month AS
SELECT 
    DATE_TRUNC('month', created_at) as month,
    complaint_type,
    COUNT(*) as complaint_count,
    AVG(EXTRACT(EPOCH FROM (closed_at - created_at))/3600) as avg_resolution_hours
FROM service_requests
WHERE complaint_type IS NOT NULL
GROUP BY DATE_TRUNC('month', created_at), complaint_type
ORDER BY month DESC, complaint_count DESC;

-- Create index on materialized view
CREATE INDEX IF NOT EXISTS idx_mv_complaints_type_month 
    ON mv_complaints_by_type_month(month, complaint_type);

-- Grant permissions (adjust as needed for production)
-- These will be executed if the roles exist
DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'ingest_role') THEN
        GRANT INSERT, SELECT ON service_requests TO ingest_role;
        GRANT SELECT, UPDATE ON etl_watermarks TO ingest_role;
    END IF;
    
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'report_role') THEN
        GRANT SELECT ON service_requests TO report_role;
        GRANT SELECT ON mv_complaints_by_day_borough TO report_role;
        GRANT SELECT ON mv_complaints_by_type_month TO report_role;
        GRANT SELECT ON etl_watermarks TO report_role;
    END IF;
END
$$;

-- Log initialization
DO $$
BEGIN
    RAISE NOTICE 'UrbanFlux schema initialized successfully';
END
$$;
