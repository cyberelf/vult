#!/bin/bash
# Quick quality gate checks for Vult development
# Usage: ./scripts/quick_check.sh [frontend|backend|all]

set -e  # Exit on error

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print with color
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Backend checks
check_backend() {
    print_step "Running backend quality checks..."
    
    print_step "1. Clippy (linting)..."
    if cargo clippy --features "cli gui" -- -D warnings; then
        print_success "Clippy passed"
    else
        print_error "Clippy failed"
        return 1
    fi
    
    print_step "2. Formatting check..."
    if cargo fmt --check; then
        print_success "Formatting check passed"
    else
        print_error "Formatting check failed - run 'cargo fmt' to fix"
        return 1
    fi
    
    print_step "3. Type checking..."
    if cargo check --features "cli gui"; then
        print_success "Type check passed"
    else
        print_error "Type check failed"
        return 1
    fi
    
    print_step "4. Running tests..."
    if cargo test --features "cli gui" --lib; then
        print_success "Tests passed"
    else
        print_error "Tests failed"
        return 1
    fi
    
    print_success "All backend checks passed!"
}

# Frontend checks
check_frontend() {
    print_step "Running frontend quality checks..."
    
    cd ui-sveltekit
    
    print_step "1. Type checking..."
    if npm run check; then
        print_success "Type check passed"
    else
        print_error "Type check failed"
        cd ..
        return 1
    fi
    
    print_step "2. Building..."
    if npm run build; then
        print_success "Build passed"
    else
        print_error "Build failed"
        cd ..
        return 1
    fi
    
    cd ..
    print_success "All frontend checks passed!"
}

# Main script
MODE=${1:-all}

case "$MODE" in
    backend)
        check_backend
        ;;
    frontend)
        check_frontend
        ;;
    all)
        check_backend
        echo ""
        check_frontend
        ;;
    *)
        echo "Usage: $0 [frontend|backend|all]"
        echo ""
        echo "  backend   Run Rust checks (clippy, fmt, type check, tests)"
        echo "  frontend  Run frontend checks (type check, build)"
        echo "  all       Run all checks (default)"
        exit 1
        ;;
esac

echo ""
print_success "✅ All quality gates passed!"
