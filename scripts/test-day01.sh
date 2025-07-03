#!/bin/bash

# Simple test script for Day 1 - No heavy dependencies
echo "🦀 Testing Day 1: Rust Basics (Lightweight)"
echo "==========================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

echo "📝 Checking Rust syntax without compilation..."

# Use rustc to check syntax only (no linking, very fast)
echo "   Checking morning examples..."
rustc --crate-type lib src/day01/morning/mod.rs --emit=metadata -o /tmp/morning_check 2>/dev/null
if [ $? -eq 0 ]; then
    echo "   ✅ Morning examples: Syntax OK"
else
    echo "   ❌ Morning examples: Syntax errors found"
    rustc --crate-type lib src/day01/morning/mod.rs --emit=metadata -o /tmp/morning_check
fi

echo "   Checking afternoon examples..."
rustc --crate-type lib src/day01/afternoon/mod.rs --emit=metadata -o /tmp/afternoon_check 2>/dev/null
if [ $? -eq 0 ]; then
    echo "   ✅ Afternoon examples: Syntax OK"
else
    echo "   ❌ Afternoon examples: Syntax errors found"
    rustc --crate-type lib src/day01/afternoon/mod.rs --emit=metadata -o /tmp/afternoon_check
fi

# Clean up temporary files
rm -f /tmp/morning_check /tmp/afternoon_check

echo ""
echo "🎯 Day 1 Status: Ready for learning!"
echo "📖 Next steps:"
echo "   1. Read docs/day01-guide.md"
echo "   2. Study src/day01/morning/mod.rs"
echo "   3. Study src/day01/afternoon/mod.rs"
echo "   4. When ready to run: cargo run --bin day01"
echo ""
echo "💡 Tip: Start by reading the code examples and understanding the concepts"
echo "    before running any compilation!"
