// Day 4 Standalone Runner
// Run with: cargo run --bin day04

mod morning;
mod afternoon;

fn main() {
    println!("🦀 Day 4: Solana Architecture & Accounts");
    println!("========================================");
    
    println!("\n📅 Morning: Solana Architecture");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Solana Accounts");
    afternoon::run_afternoon_examples();
} 