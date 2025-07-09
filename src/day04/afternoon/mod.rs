// Day 4 Afternoon: Solana Accounts
// Concepts: Account data layout, ownership, rent, practice

pub fn run_afternoon_examples() {
    println!("🌅 Afternoon Lesson: Solana Accounts");
    println!("-----------------------------------");

    account_data_layout();
    account_ownership();
    account_rent();
    practice_exercises();

    println!("\n✅ Afternoon exercises completed!");
    println!("💡 Practice: Try describing an account's data layout in your own words!");
}

fn account_data_layout() {
    println!("\n📦 1. Account Data Layout:");
    println!("   - Each account has a fixed-size data field (can store bytes).");
    println!("   - Used to store user data, program state, or code.");
    println!("   - Data is just a sequence of bytes; programs decide how to interpret it.");
}

fn account_ownership() {
    println!("\n🔑 2. Account Ownership:");
    println!("   - Every account has an owner (usually a program or the system program).");
    println!("   - Only the owner program can change the account's data.");
    println!("   - Ownership is set when the account is created.");
}

fn account_rent() {
    println!("\n💰 3. Account Rent:");
    println!("   - Accounts must pay rent to stay alive on the blockchain.");
    println!("   - If an account's balance is too low, it may be deleted (reclaimed).");
    println!("   - You can make an account 'rent-exempt' by funding it with enough SOL.");
}

fn practice_exercises() {
    println!("\n🎯 Practice Exercises:");
    println!("   1. Draw a diagram of a Solana account, labeling: address, owner, data, lamports (SOL), and rent status.");
    println!("   2. Explain in your own words why only the owner can change account data.");
    println!("   3. What happens if an account runs out of SOL for rent?");
} 