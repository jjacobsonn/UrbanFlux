#!/bin/bash
# UrbanFlux Setup & Management Script
# Professional installation and management automation

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="UrbanFlux"
POSTGRES_CONTAINER="urbanflux-postgres"
DB_NAME="urbanflux"
DB_USER="urbanflux_user"
DB_PASSWORD="urbanflux_dev_password"

# Print colored messages
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_header() {
    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}\n"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check system dependencies
check_dependencies() {
    print_header "Checking System Dependencies"
    
    local missing_deps=()
    
    # Check Docker
    if command_exists docker; then
        print_success "Docker is installed ($(docker --version | cut -d' ' -f3 | tr -d ','))"
    else
        print_error "Docker is not installed"
        missing_deps+=("docker")
    fi
    
    # Check Docker Compose
    if command_exists docker && docker compose version >/dev/null 2>&1; then
        print_success "Docker Compose is installed"
    else
        print_error "Docker Compose is not installed"
        missing_deps+=("docker-compose")
    fi
    
    # Check Rust/Cargo
    if command_exists cargo; then
        print_success "Rust is installed ($(cargo --version | cut -d' ' -f2))"
    else
        print_error "Rust is not installed"
        missing_deps+=("rust")
    fi
    
    # Check pkg-config (optional but recommended)
    if command_exists pkg-config; then
        print_success "pkg-config is installed"
    else
        print_warning "pkg-config is not installed (optional)"
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        echo ""
        print_info "Installation instructions:"
        for dep in "${missing_deps[@]}"; do
            case $dep in
                docker)
                    echo "  Docker: https://docs.docker.com/get-docker/"
                    ;;
                docker-compose)
                    echo "  Docker Compose: Included with Docker Desktop or install separately"
                    ;;
                rust)
                    echo "  Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                    ;;
            esac
        done
        return 1
    fi
    
    print_success "All required dependencies are installed"
    return 0
}

# Setup environment variables
setup_env() {
    print_header "Setting Up Environment"
    
    if [ ! -f .env ]; then
        if [ -f .env.example ]; then
            cp .env.example .env
            print_success "Created .env from .env.example"
        else
            cat > .env <<EOF
# Database Configuration
PGHOST=localhost
PGPORT=5432
PGUSER=$DB_USER
PGPASSWORD=$DB_PASSWORD
PGDATABASE=$DB_NAME

# ETL Configuration
ETL_CHUNK_SIZE=100000
ETL_MODE=full
ETL_BAD_ROWS_DIR=/app/bad_rows
ETL_RUNS_DIR=/app/runs

# Logging
RUST_LOG=info
LOG_FORMAT=pretty
EOF
            print_success "Created default .env file"
        fi
    else
        print_info ".env file already exists"
    fi
    
    # Export variables for current session
    export PGHOST=localhost
    export PGPORT=5432
    export PGUSER=$DB_USER
    export PGPASSWORD=$DB_PASSWORD
    export PGDATABASE=$DB_NAME
    export RUST_LOG=info
    
    print_success "Environment variables configured"
}

# Build Rust project
build_project() {
    print_header "Building Rust Project"
    
    print_info "Running cargo build (this may take a few minutes on first run)..."
    if cargo build --release 2>&1 | tee /tmp/urbanflux-build.log | grep -E "(Compiling|Finished)"; then
        print_success "Project built successfully"
        return 0
    else
        print_error "Build failed. Check /tmp/urbanflux-build.log for details"
        return 1
    fi
}

# Start Docker containers
start_docker() {
    print_header "Starting Docker Containers"
    
    # Stop any existing containers
    if docker ps -a | grep -q $POSTGRES_CONTAINER; then
        print_info "Stopping existing containers..."
        docker compose down 2>/dev/null || true
    fi
    
    print_info "Starting PostgreSQL container..."
    if docker compose up -d postgres; then
        print_success "PostgreSQL container started"
        
        # Wait for PostgreSQL to be ready
        print_info "Waiting for PostgreSQL to be ready..."
        for i in {1..30}; do
            if docker exec $POSTGRES_CONTAINER pg_isready -U $DB_USER -d $DB_NAME >/dev/null 2>&1; then
                print_success "PostgreSQL is ready"
                return 0
            fi
            sleep 1
        done
        print_error "PostgreSQL failed to start within 30 seconds"
        return 1
    else
        print_error "Failed to start PostgreSQL container"
        return 1
    fi
}

# Run database migrations
run_migrations() {
    print_header "Running Database Migrations"
    
    if cargo run --release -- db migrate; then
        print_success "Migrations completed successfully"
        return 0
    else
        print_error "Migrations failed"
        print_info "If you continue to see errors, try: ./setup.sh and select option 11 (Complete Reset)"
        return 1
    fi
}

# Load test data
load_test_data() {
    print_header "Loading Test Data"
    
    if [ ! -f testdata/sample.csv ]; then
        print_error "testdata/sample.csv not found"
        return 1
    fi
    
    print_info "Running ETL pipeline with test data..."
    if cargo run --release -- run --mode full --input testdata/sample.csv; then
        print_success "Test data loaded successfully"
        
        # Show record count
        local count=$(cargo run --release --quiet -- report last-run 2>/dev/null | grep -oP 'Total records.*: \K\d+' || echo "unknown")
        print_info "Records in database: $count"
        return 0
    else
        print_error "Failed to load test data"
        return 1
    fi
}

# Refresh materialized views
refresh_views() {
    print_header "Refreshing Materialized Views"
    
    if cargo run --release -- db refresh-mv --concurrently; then
        print_success "Materialized views refreshed"
        return 0
    else
        print_error "Failed to refresh materialized views"
        return 1
    fi
}

# Run tests
run_tests() {
    print_header "Running Unit Tests"
    
    if cargo test --lib -- --nocapture; then
        print_success "All tests passed"
        return 0
    else
        print_error "Some tests failed"
        return 1
    fi
}

# Check database health
health_check() {
    print_header "Health Check"
    
    # Check Docker container
    if docker ps | grep -q $POSTGRES_CONTAINER; then
        print_success "PostgreSQL container is running"
    else
        print_error "PostgreSQL container is not running"
        return 1
    fi
    
    # Check database connection
    if cargo run --release -- db health >/dev/null 2>&1; then
        print_success "Database connection is healthy"
    else
        print_error "Cannot connect to database"
        return 1
    fi
    
    # Show statistics
    print_info "Database statistics:"
    cargo run --release --quiet -- report last-run 2>/dev/null || print_warning "No ETL runs recorded yet"
    
    return 0
}

# Query data samples
query_samples() {
    print_header "Sample Data Query"
    
    print_info "Service Requests (last 5):"
    docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d $DB_NAME -c \
        "SELECT unique_key, complaint_type, borough, created_at FROM service_requests ORDER BY created_at DESC LIMIT 5;"
    
    print_info "\nComplaints by Borough:"
    docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d $DB_NAME -c \
        "SELECT * FROM mv_complaints_by_day_borough ORDER BY complaint_date DESC, complaint_count DESC LIMIT 10;"
}

# Complete reset
reset_all() {
    print_header "Complete Reset"
    
    read -p "This will delete ALL data and containers. Are you sure? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        print_info "Reset cancelled"
        return 0
    fi
    
    print_warning "Dropping database to clear migration history..."
    if docker ps | grep -q $POSTGRES_CONTAINER; then
        docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d postgres -c "DROP DATABASE IF EXISTS $DB_NAME;" 2>/dev/null || true
        docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d postgres -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || true
        print_success "Database reset complete"
    fi
    
    print_warning "Stopping and removing containers..."
    docker compose down -v 2>/dev/null || true
    
    print_warning "Cleaning build artifacts..."
    cargo clean
    
    print_warning "Removing .env file..."
    rm -f .env
    
    print_success "Reset complete. Run full installation to set up again."
}

# Stop services
stop_services() {
    print_header "Stopping Services"
    
    if docker compose down; then
        print_success "All services stopped"
    else
        print_error "Failed to stop services"
        return 1
    fi
}

# Full installation
full_install() {
    print_header "🚀 Full Installation: $PROJECT_NAME"
    
    check_dependencies || exit 1
    setup_env || exit 1
    build_project || exit 1
    start_docker || exit 1
    sleep 2  # Give PostgreSQL extra time
    
    # Ensure clean database state to prevent migration version conflicts
    print_info "Ensuring clean database state..."
    docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d postgres -c "DROP DATABASE IF EXISTS $DB_NAME;" >/dev/null 2>&1 || true
    docker exec $POSTGRES_CONTAINER psql -U $DB_USER -d postgres -c "CREATE DATABASE $DB_NAME;" >/dev/null 2>&1 || true
    print_success "Database initialized"
    
    run_migrations || exit 1
    load_test_data || exit 1
    refresh_views || exit 1
    
    print_header "✨ Installation Complete!"
    print_success "UrbanFlux is ready to use"
    print_info "\nQuick commands:"
    echo "  ./setup.sh menu      - Show interactive menu"
    echo "  ./setup.sh health    - Check system health"
    echo "  ./setup.sh query     - View sample data"
    echo "  ./setup.sh stop      - Stop services"
    echo "  ./setup.sh reset     - Complete reset"
}

# Interactive menu
show_menu() {
    while true; do
        clear
        print_header "🏙️  $PROJECT_NAME - Management Menu"
        echo "1)  📦 Full Installation (Fresh Start)"
        echo "2)  🏗️  Build Project Only"
        echo "3)  🐳 Start Docker Containers"
        echo "4)  🗄️  Run Database Migrations"
        echo "5)  📊 Load Test Data"
        echo "6)  🔄 Refresh Materialized Views"
        echo "7)  🧪 Run Tests"
        echo "8)  ❤️  Health Check"
        echo "9)  🔍 Query Sample Data"
        echo "10) 🛑 Stop Services"
        echo "11) 🗑️  Complete Reset"
        echo "12) 📋 Show Logs"
        echo "13) 🚪 Exit"
        echo ""
        read -p "Select option (1-13): " choice
        
        case $choice in
            1) full_install ;;
            2) build_project ;;
            3) start_docker ;;
            4) run_migrations ;;
            5) load_test_data ;;
            6) refresh_views ;;
            7) run_tests ;;
            8) health_check ;;
            9) query_samples ;;
            10) stop_services ;;
            11) reset_all ;;
            12) docker compose logs -f ;;
            13) print_info "Goodbye!"; exit 0 ;;
            *) print_error "Invalid option" ;;
        esac
        
        echo ""
        read -p "Press Enter to continue..."
    done
}

# Main entry point
main() {
    case "${1:-menu}" in
        install|setup)
            full_install
            ;;
        build)
            build_project
            ;;
        start)
            start_docker
            ;;
        migrate)
            run_migrations
            ;;
        load)
            load_test_data
            ;;
        refresh)
            refresh_views
            ;;
        test)
            run_tests
            ;;
        health|status)
            health_check
            ;;
        query)
            query_samples
            ;;
        stop)
            stop_services
            ;;
        reset)
            reset_all
            ;;
        menu)
            show_menu
            ;;
        help|--help|-h)
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  install  - Full installation"
            echo "  build    - Build Rust project"
            echo "  start    - Start Docker containers"
            echo "  migrate  - Run database migrations"
            echo "  load     - Load test data"
            echo "  refresh  - Refresh materialized views"
            echo "  test     - Run unit tests"
            echo "  health   - Health check"
            echo "  query    - Query sample data"
            echo "  stop     - Stop services"
            echo "  reset    - Complete reset"
            echo "  menu     - Interactive menu (default)"
            echo "  help     - Show this help"
            ;;
        *)
            print_error "Unknown command: $1"
            echo "Run '$0 help' for usage information"
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
