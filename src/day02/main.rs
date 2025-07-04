// Day 2 Standalone Runner
// Run with: cargo run --bin day02

mod morning;
mod afternoon;

fn main() {
    println!("🦀 Day 2: Control Flow & Data Structures");
    println!("========================================");
    
    println!("\n📅 Morning: Control Flow & Functions");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Structs, Enums & Collections");
    afternoon::run_afternoon_examples();
}
