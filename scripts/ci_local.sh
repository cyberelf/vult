#!/bin/bash
# Local CI checks - mirrors GitHub Actions workflow
# Run this before pushing to ensure CI will pass
# Usage: ./scripts/ci_local.sh [--skip-slow]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

SKIP_SLOW=false
if [[ "$1" == "--skip-slow" ]]; then
    SKIP_SLOW=true
fi

print_header() {
    echo -e "\n${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}  $1"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}\n"
}

print_step() {
    echo -e "${BLUE}▶${NC} $1"
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

# Track failures
FAILED_CHECKS=()

run_check() {
    local name="$1"
    shift
    
    if "$@"; then
        print_success "$name passed"
        return 0
    else
        print_error "$name failed"
        FAILED_CHECKS+=("$name")
        return 1
    fi
}

# ============================================================================
# CHECK JOB - Format, lint, and build
# ============================================================================
print_header "CHECK JOB"

print_step "Checking code formatting..."
if cargo fmt --all -- --check; then
    print_success "Formatting check passed"
else
    print_error "Formatting check failed - running cargo fmt to fix"
    cargo fmt --all
    print_warning "Code reformatted - review changes before committing"
fi

print_step "Running clippy (linter)..."
run_check "Clippy" cargo clippy --lib --features gui -- -D warnings || true

print_step "Building library..."
run_check "Library build" cargo build --lib --release || true

print_step "Building CLI..."
run_check "CLI build" cargo build --bin vult --features cli --release || true

print_step "Building GUI..."
run_check "GUI build" cargo build --bin vult-gui --features gui --release || true

# ============================================================================
# TEST JOB - Run test suite
# ============================================================================
print_header "TEST JOB"

print_step "Running library tests..."
run_check "Library tests" cargo test --lib || true

print_step "Running integration tests..."
run_check "Integration tests" cargo test --test integration_test || true

# ============================================================================
# DENY JOB - Dependency check
# ============================================================================
if [ "$SKIP_SLOW" = false ]; then
    print_header "DEPENDENCY CHECK JOB"
    
    print_step "Checking if cargo-deny is installed..."
    if ! command -v cargo-deny &> /dev/null; then
        print_warning "cargo-deny not installed, installing..."
        cargo install cargo-deny
    fi
    
    print_step "Running dependency check..."
    # Note: CI allows this to fail, we do too
    if cargo deny check; then
        print_success "Dependency check passed"
    else
        print_warning "Dependency check had warnings (non-blocking in CI)"
    fi
else
    print_warning "Skipping dependency check (--skip-slow enabled)"
fi

# ============================================================================
# SUMMARY
# ============================================================================
print_header "SUMMARY"

if [ ${#FAILED_CHECKS[@]} -eq 0 ]; then
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✓ ALL CHECKS PASSED - Ready to push!                       ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║  ✗ FAILED CHECKS:                                            ║${NC}"
    for check in "${FAILED_CHECKS[@]}"; do
        echo -e "${RED}║    - $check${NC}"
    done
    echo -e "${RED}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    print_error "Fix the issues above before pushing"
    exit 1
fi
