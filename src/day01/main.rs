// Day 1 Standalone Runner
// Run with: cargo run --bin day01

mod morning;
mod afternoon;

fn main() {
    println!("🦀 Day 1: Rust Basics - Syntax & Ownership");
    println!("==========================================");
    
    println!("\n📅 Morning: Core Syntax & Variables");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Ownership, Borrowing & Lifetimes");
    afternoon::run_afternoon_examples();
}
