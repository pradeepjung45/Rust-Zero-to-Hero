#!/bin/bash

# Setup verification script for Rust & Solana development environment

echo "🔍 Checking Rust & Solana Development Environment..."
echo "=================================================="

# Check Rust installation
echo -n "Checking Rust... "
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ $RUST_VERSION"
else
    echo "❌ Rust not found. Please install from https://rustup.rs/"
    exit 1
fi

# Check Cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "✅ $CARGO_VERSION"
else
    echo "❌ Cargo not found. Should come with Rust installation."
    exit 1
fi

# Check Solana CLI (optional for early days)
echo -n "Checking Solana CLI... "
if command -v solana &> /dev/null; then
    SOLANA_VERSION=$(solana --version)
    echo "✅ $SOLANA_VERSION"
else
    echo "⚠️  Solana CLI not found. Install later for Days 4+:"
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/v1.18.4/install)\""
fi

# Check Anchor (optional for early days)
echo -n "Checking Anchor... "
if command -v anchor &> /dev/null; then
    ANCHOR_VERSION=$(anchor --version)
    echo "✅ $ANCHOR_VERSION"
else
    echo "⚠️  Anchor not found. Install later for Days 6+:"
    echo "   npm install -g @coral-xyz/anchor-cli"
fi

# Check Node.js (needed for Anchor)
echo -n "Checking Node.js... "
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "✅ $NODE_VERSION"
else
    echo "⚠️  Node.js not found. Install from https://nodejs.org/ (needed for Days 6+)"
fi

echo ""
echo "🎯 Environment Status:"
echo "- Rust & Cargo: Ready for Days 1-5"
if command -v solana &> /dev/null && command -v anchor &> /dev/null; then
    echo "- Solana & Anchor: Ready for Days 6-12"
else
    echo "- Solana & Anchor: Install before Day 4 and Day 6 respectively"
fi

echo ""
echo "🚀 Ready to start learning! Begin with:"
echo "   cargo run --bin day01"
