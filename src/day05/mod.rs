// Day 5: Transactions & Programs
// Uncomment modules as you complete each lesson

pub mod morning;    // Transactions
pub mod afternoon;  // Programs

pub fn run_examples() {
    println!("🦀 Day 5: Transactions & Programs");
    println!("================================");
    
    println!("\n📅 Morning: Transactions");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Programs");
    afternoon::run_afternoon_examples();
} 