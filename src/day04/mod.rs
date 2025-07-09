// Day 4: Solana Architecture & Accounts
// Uncomment modules as you complete each lesson

pub mod morning;    // Solana Architecture
pub mod afternoon;  // Solana Accounts

pub fn run_examples() {
    println!("🦀 Day 4: Solana Architecture & Accounts");
    println!("========================================");
    
    println!("\n📅 Morning: Solana Architecture");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Solana Accounts");
    afternoon::run_afternoon_examples();
} 