// Day 3: Traits, Generics & Error Handling
// Uncomment modules as you complete each lesson

pub mod morning;    // Traits & Generics
pub mod afternoon;  // Advanced Error Handling & Modules

pub fn run_examples() {
    println!("🦀 Day 3: Traits, Generics & Error Handling");
    println!("===========================================");
    
    println!("\n📅 Morning: Traits & Generics");
    morning::run_morning_examples();
    
    println!("\n🌅 Afternoon: Advanced Error Handling & Modules");
    afternoon::run_afternoon_examples();
}
