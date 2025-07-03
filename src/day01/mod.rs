// Day 1: Rust Basics - Syntax & Ownership
// Uncomment modules as you complete each lesson

pub mod morning;    // Core Syntax & Variables
pub mod afternoon;  // Ownership, Borrowing & Lifetimes

pub fn run_examples() {
    println!("🦀 Day 1: Rust Basics - Syntax & Ownership");
    println!("==========================================");
    
    println!("\n📅 Morning: Core Syntax & Variables");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Ownership, Borrowing & Lifetimes");
    afternoon::run_afternoon_examples();
}
