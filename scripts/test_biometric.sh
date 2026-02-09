#!/bin/bash
# Test script to verify windows-biometric feature is working

echo "Testing Windows Hello Integration..."
echo "===================================="
echo ""

# Build the project with default features (should include windows-biometric via gui feature)
echo "1. Building with default features (gui enabled)..."
cargo build --quiet --bin vult-gui 2>&1 | grep -E "(windows-biometric|ENABLED|DISABLED)"

echo ""
echo "2. Running unit tests for biometric availability..."
cargo test --quiet biometric_availability_test 2>&1 | grep -E "(passed|failed|ok|Available|NotSupported)"

echo ""
echo "3. Feature check complete."
echo ""
echo "To run the GUI app with debug output:"
echo "  cargo run --bin vult-gui"
echo ""
