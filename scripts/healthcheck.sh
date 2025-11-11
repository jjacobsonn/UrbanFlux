#!/bin/bash
# Simple health check script for Docker healthchecks

set -e

# Check if PostgreSQL is accepting connections
psql -U "${PGUSER:-urbanflux_user}" -d "${PGDATABASE:-urbanflux}" -c "SELECT 1" > /dev/null 2>&1

echo "PostgreSQL is healthy"
