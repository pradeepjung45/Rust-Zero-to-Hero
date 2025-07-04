// Rust & Solana Development: Zero to Hero
// Main entry point - uncomment modules as you progress through the course

// Phase 1: Rust Fundamentals (Days 1-3)
mod day01;  // Day 1: Rust Basics - Syntax & Ownership
mod day02;  // Day 2: Control Flow & Data Structures  
// mod day03;  // Day 3: Traits, Generics & Error Handling

// Phase 2: Solana Blockchain Basics (Days 4-5)
// mod day04;  // Day 4: Solana Architecture & Accounts
// mod day05;  // Day 5: Transactions & Programs

// Phase 3: Anchor Framework Introduction (Days 6-7)
// Note: Days 6+ will use separate Anchor workspace structure

// Phase 4: Building Simple Solana Programs (Days 8-9)
// Note: These will be Anchor programs in the programs/ directory

// Phase 5: Advanced Topics (Days 10-12)
// Note: Advanced topics will be in separate Anchor programs

fn main() {
    println!("🦀 Welcome to Rust & Solana Development: Zero to Hero! 🚀");
    println!();
    println!("📚 Course Overview:");
    println!("Phase 1 (Days 1-3): Rust Fundamentals - START HERE!");
    println!("Phase 2 (Days 4-5): Solana Blockchain Basics");
    println!("Phase 3 (Days 6-7): Anchor Framework Introduction");
    println!("Phase 4 (Days 8-9): Building Simple Solana Programs");
    println!("Phase 5 (Days 10-12): Advanced Topics & Best Practices");
    println!();
    println!("🎯 Step-by-Step Learning (No Heavy Dependencies!):");
    println!("1. Read docs/day01-guide.md and docs/day02-guide.md for detailed explanations");
    println!("2. Study code examples in src/day01/ and src/day02/ directories");
    println!("3. Practice by modifying the examples");
    println!("4. Run individual days: cargo run --bin day01 or cargo run --bin day02");
    println!();
    println!("📖 Current Status: Day 1 ✅ and Day 2 ✅ are ready for learning!");
    println!("💡 Tip: Focus on understanding concepts before running code");

    // Uncomment these as you progress through the course:

    // Day 1 examples (uncomment when you want to run them)
    day01::run_examples();

    // Future days will be added as you progress
    day02::run_examples();
    // day03::run_examples();
    // etc.
}
