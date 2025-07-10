// Day 5 Standalone Runner
// Run with: cargo run --bin day05

mod morning;
mod afternoon;

fn main() {
    println!("🦀 Day 5: Transactions & Programs");
    println!("================================");
    
    println!("\n📅 Morning: Transactions");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Programs");
    afternoon::run_afternoon_examples();
} 