// Day 4 Morning: Solana Architecture
// Concepts: Solana clusters, validators, accounts, programs

pub fn run_morning_examples() {
    println!("🌅 Morning Lesson: Solana Architecture");
    println!("--------------------------------------");

    solana_clusters();
    solana_validators();
    solana_accounts_intro();
    solana_programs_intro();

    println!("\n✅ Morning exercises completed!");
    println!("💡 Practice: Try drawing a diagram of Solana's architecture!");
}

fn solana_clusters() {
    println!("\n🌐 1. Solana Clusters:");
    println!("   - A cluster is a set of validators working together.");
    println!("   - Mainnet Beta, Testnet, Devnet are different clusters.");
    println!("   - Each cluster maintains its own ledger (blockchain).");
}

fn solana_validators() {
    println!("\n🖥️ 2. Validators:");
    println!("   - Validators are computers that process transactions and add blocks.");
    println!("   - They vote on the order of transactions and maintain the ledger.");
    println!("   - Anyone can run a validator node.");
}

fn solana_accounts_intro() {
    println!("\n📦 3. Accounts:");
    println!("   - Accounts store data and SOL tokens.");
    println!("   - Every account has a public key (address).");
    println!("   - Programs (smart contracts) and users both use accounts.");
}

fn solana_programs_intro() {
    println!("\n📝 4. Programs:");
    println!("   - Programs are smart contracts deployed to the blockchain.");
    println!("   - Programs are also stored in accounts.");
    println!("   - Users interact with programs by sending transactions.");
} 