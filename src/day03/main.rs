// Day 3 Standalone Runner
// Run with: cargo run --bin day03

mod morning;
mod afternoon;

fn main() {
    println!("🦀 Day 3: Traits, Generics & Error Handling");
    println!("===========================================");
    
    println!("\n📅 Morning: Traits & Generics");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Advanced Error Handling & Modules");
    afternoon::run_afternoon_examples();
} 