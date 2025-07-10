// Day 5 Morning: Transactions
// Concepts: What is a transaction, instructions, signatures

pub fn run_morning_examples() {
    println!("🌅 Morning Lesson: Transactions");
    println!("-------------------------------");

    what_is_transaction();
    transaction_instructions();
    transaction_signatures();

    println!("\n✅ Morning exercises completed!");
    println!("💡 Practice: Try describing a transaction in your own words!");
}

fn what_is_transaction() {
    println!("\n💸 1. What is a Transaction?:");
    println!("   - A transaction is a message sent to the Solana blockchain to do something (like transfer SOL or interact with a program).");
    println!("   - It contains all the info needed for the network to process your request.");
}

fn transaction_instructions() {
    println!("\n📝 2. Instructions:");
    println!("   - Each transaction can have one or more instructions.");
    println!("   - An instruction tells a program what to do (like 'transfer tokens' or 'update data').");
    println!("   - Instructions specify which accounts are involved and what action to take.");
}

fn transaction_signatures() {
    println!("\n✍️ 3. Signatures:");
    println!("   - Every transaction must be signed by the account(s) authorizing it.");
    println!("   - Signatures prove you have permission to spend tokens or change data.");
    println!("   - The network checks signatures before processing the transaction.");
} 