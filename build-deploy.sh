#!/usr/bin/env bash

# Brain Coin - Complete Deployment Script for WSL/Linux
# Copy and paste these commands into your WSL terminal to build and prepare for deployment

echo "=========================================="
echo "Brain Coin Devnet Deployment"
echo "=========================================="
echo ""

# Step 1: Install Rust & Cargo (if not already installed)
echo "Step 1: Setting up Rust build environment..."
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✓ Rust already installed"
fi

# Step 2: Install cargo-build-sbf
echo "Step 2: Installing cargo-build-sbf..."
cargo install cargo-build-sbf

# Step 3: Navigate to project directory
echo "Step 3: Setting up project directory..."
cd /mnt/c/Users/YourUser/Documents/braincoin  # CHANGE THIS PATH
# Or if using WSL with mounted drive:
# cd /mnt/c/Users/[YourUsername]/Documents/braincoin

# Step 4: Build all programs
echo "Step 4: Building Solana programs..."
echo ""

echo "  Building Brain Program..."
cargo build-sbf --manifest-path programs/brain/Cargo.toml --release
if [ $? -eq 0 ]; then echo "  ✓ Brain program built successfully"; else echo "  ✗ Brain build failed"; exit 1; fi
echo ""

echo "  Building Guardian Program..."
cargo build-sbf --manifest-path programs/guardian/Cargo.toml --release
if [ $? -eq 0 ]; then echo "  ✓ Guardian program built successfully"; else echo "  ✗ Guardian build failed"; exit 1; fi
echo ""

echo "  Building Agent Syndicate Program..."
cargo build-sbf --manifest-path programs/agent-syndicate/Cargo.toml --release
if [ $? -eq 0 ]; then echo "  ✓ Agent Syndicate program built successfully"; else echo "  ✗ Agent Syndicate build failed"; exit 1; fi
echo ""

echo "  Building Fee Collector Program..."
cargo build-sbf --manifest-path programs/fee-collector/Cargo.toml --release
if [ $? -eq 0 ]; then echo "  ✓ Fee Collector program built successfully"; else echo "  ✗ Fee Collector build failed"; exit 1; fi
echo ""

# Step 5: Verify builds
echo "Step 5: Verifying build artifacts..."
if [ -f "target/sbf-solana-solana/release/brain.so" ]; then
    echo "✓ brain.so generated"
else
    echo "✗ brain.so NOT FOUND"
    exit 1
fi

if [ -f "target/sbf-solana-solana/release/guardian.so" ]; then
    echo "✓ guardian.so generated"
else
    echo "✗ guardian.so NOT FOUND"
    exit 1
fi

if [ -f "target/sbf-solana-solana/release/agent_syndicate.so" ]; then
    echo "✓ agent_syndicate.so generated"
else
    echo "✗ agent_syndicate.so NOT FOUND"
    exit 1
fi

if [ -f "target/sbf-solana-solana/release/fee_collector.so" ]; then
    echo "✓ fee_collector.so generated"
else
    echo "✗ fee_collector.so NOT FOUND"
    exit 1
fi

echo ""
echo "=========================================="
echo "✅ All programs built successfully!"
echo "=========================================="
echo ""
echo "Next steps (run in PowerShell on Windows):"
echo ""
echo "1. Deploy to devnet:"
echo "   solana program deploy target/sbf-solana-solana/release/brain.so -u devnet"
echo "   solana program deploy target/sbf-solana-solana/release/guardian.so -u devnet"
echo "   solana program deploy target/sbf-solana-solana/release/agent_syndicate.so -u devnet"
echo "   solana program deploy target/sbf-solana-solana/release/fee_collector.so -u devnet"
echo ""
echo "2. Record the Program IDs from the output above"
echo ""
echo "3. Update Anchor.toml with the Program IDs"
echo ""
echo "4. Create .env file with configuration"
echo ""
echo "5. Start services:"
echo "   npm run brain-agent"
echo "   npm run reward-distributor"
echo "   npm run dev-buy"
