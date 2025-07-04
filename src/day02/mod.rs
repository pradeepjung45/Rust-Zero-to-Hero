// Day 2: Control Flow & Data Structures

pub mod morning;    // Control Flow
pub mod afternoon;  // Data Structures

pub fn run_examples() {
    println!("\n🦀 Day 2: Control Flow & Data Structures");
    println!("==========================================");
    
    println!("\n📅 Morning: Control Flow & Functions");
    morning::run_morning_examples();

    println!("\n🌅 Afternoon: Structs, Enums & Collections");
    afternoon::run_afternoon_examples();
}

