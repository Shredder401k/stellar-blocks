#!/bin/bash

# Soroban Smart Contract Environment Setup Script
# This script installs and configures the necessary tools for Soroban development

set -e

echo "🚀 Setting up Soroban development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust is already installed ($(rustc --version))"
fi

# Add wasm32 target
echo "📦 Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
if ! command -v soroban &> /dev/null; then
    echo "📦 Installing Soroban CLI..."
    cargo install --locked soroban-cli
else
    echo "✅ Soroban CLI is already installed ($(soroban --version))"
fi

# Configure Soroban for testnet
echo "⚙️  Configuring Soroban for Stellar testnet..."
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" 2>/dev/null || echo "Testnet already configured"

# Create a default identity if it doesn't exist
if ! soroban config identity ls | grep -q "default"; then
    echo "🔑 Creating default identity..."
    soroban config identity generate --global default
    echo "💰 Funding default identity on testnet..."
    soroban config identity fund default --network testnet
else
    echo "✅ Default identity already exists"
fi

echo ""
echo "✨ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. cd contracts/example"
echo "  2. cargo build --target wasm32-unknown-unknown --release"
echo "  3. cargo test"
echo ""
echo "To deploy to testnet:"
echo "  soroban contract deploy \\"
echo "    --wasm target/wasm32-unknown-unknown/release/example.wasm \\"
echo "    --source default \\"
echo "    --network testnet"
echo ""
