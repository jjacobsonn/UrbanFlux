-- UrbanFlux Database Schema Migration
-- Version: 20251112000001
-- Description: Initial schema with service requests, ETL watermarks, and materialized views

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create service_requests table
CREATE TABLE IF NOT EXISTS service_requests (
    unique_key VARCHAR(50) PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    closed_at TIMESTAMP,
    agency VARCHAR(255),
    agency_name VARCHAR(255),
    complaint_type VARCHAR(255) NOT NULL,
    descriptor VARCHAR(255),
    location_type VARCHAR(255),
    incident_zip VARCHAR(10),
    incident_address VARCHAR(255),
    street_name VARCHAR(255),
    cross_street_1 VARCHAR(255),
    cross_street_2 VARCHAR(255),
    intersection_street_1 VARCHAR(255),
    intersection_street_2 VARCHAR(255),
    address_type VARCHAR(50),
    city VARCHAR(100),
    landmark VARCHAR(255),
    facility_type VARCHAR(255),
    status VARCHAR(50),
    due_date TIMESTAMP,
    resolution_description TEXT,
    resolution_action_updated_date TIMESTAMP,
    community_board VARCHAR(10),
    bbl VARCHAR(20),
    borough VARCHAR(50) NOT NULL,
    x_coordinate DOUBLE PRECISION,
    y_coordinate DOUBLE PRECISION,
    open_data_channel_type VARCHAR(50),
    park_facility_name VARCHAR(255),
    park_borough VARCHAR(50),
    vehicle_type VARCHAR(100),
    taxi_company_borough VARCHAR(50),
    taxi_pick_up_location VARCHAR(255),
    bridge_highway_name VARCHAR(255),
    bridge_highway_direction VARCHAR(50),
    road_ramp VARCHAR(255),
    bridge_highway_segment VARCHAR(255),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    location_point VARCHAR(255),
    ingested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes on service_requests for query performance
CREATE INDEX IF NOT EXISTS idx_service_requests_created_at 
    ON service_requests(created_at);

CREATE INDEX IF NOT EXISTS idx_service_requests_borough 
    ON service_requests(borough);

CREATE INDEX IF NOT EXISTS idx_service_requests_complaint_type 
    ON service_requests(complaint_type);

CREATE INDEX IF NOT EXISTS idx_service_requests_ingested_at 
    ON service_requests(ingested_at);

-- Create etl_watermarks table for tracking ETL runs
CREATE TABLE IF NOT EXISTS etl_watermarks (
    run_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    run_mode VARCHAR(50) NOT NULL,
    dataset_name VARCHAR(255),
    started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    status VARCHAR(50) NOT NULL DEFAULT 'running',
    records_processed INTEGER DEFAULT 0,
    error_message TEXT,
    rows_processed BIGINT DEFAULT 0,
    rows_inserted BIGINT DEFAULT 0,
    rows_duplicated BIGINT DEFAULT 0,
    rows_rejected BIGINT DEFAULT 0
);

-- Create indexes on etl_watermarks for tracking queries
CREATE INDEX IF NOT EXISTS idx_etl_watermarks_run_id 
    ON etl_watermarks(run_id);

CREATE INDEX IF NOT EXISTS idx_etl_watermarks_started_at 
    ON etl_watermarks(started_at DESC);

-- Create materialized view: complaints by day and borough
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_complaints_by_day_borough AS
SELECT 
    DATE(created_at) AS complaint_date,
    borough,
    COUNT(*) AS complaint_count
FROM service_requests
GROUP BY DATE(created_at), borough;

-- Create unique index on mv_complaints_by_day_borough for concurrent refresh
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_complaints_by_day_borough_unique 
    ON mv_complaints_by_day_borough(complaint_date, borough);

-- Create materialized view: complaints by type and month
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_complaints_by_type_month AS
SELECT 
    DATE_TRUNC('month', created_at) AS month,
    complaint_type,
    COUNT(*) AS complaint_count
FROM service_requests
GROUP BY DATE_TRUNC('month', created_at), complaint_type;

-- Create unique index on mv_complaints_by_type_month for concurrent refresh
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_complaints_by_type_month_unique 
    ON mv_complaints_by_type_month(month, complaint_type);

-- Add comments for documentation
COMMENT ON TABLE service_requests IS 'NYC 311 service request records from Open Data';
COMMENT ON TABLE etl_watermarks IS 'ETL execution tracking with comprehensive statistics';
COMMENT ON MATERIALIZED VIEW mv_complaints_by_day_borough IS 'Aggregated complaints by date and borough (requires concurrent refresh)';
COMMENT ON MATERIALIZED VIEW mv_complaints_by_type_month IS 'Aggregated complaints by month and type (requires concurrent refresh)';
