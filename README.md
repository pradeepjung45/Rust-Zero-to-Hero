# 🦀 Rust & Solana Development: Zero to Hero

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)

A comprehensive **12-day learning path** for mastering Rust programming and Solana blockchain development using the Anchor framework. Designed for **step-by-step learning** without heavy dependencies or long compilation times.

## 🎯 Learning Objectives

By the end of this course, you will:
- Master Rust fundamentals for blockchain development
- Understand Solana's architecture and programming model
- Build, test, and deploy Solana programs using Anchor
- Implement security best practices and advanced patterns

## 📚 Course Structure

### Phase 1: Rust Fundamentals (Days 1-3)
- **Day 1**: Rust Basics - Syntax & Ownership
- **Day 2**: Control Flow & Data Structures  
- **Day 3**: Traits, Generics & Error Handling

### Phase 2: Solana Blockchain Basics (Days 4-5)
- **Day 4**: Solana Architecture & Accounts
- **Day 5**: Transactions & Programs

### Phase 3: Anchor Framework Introduction (Days 6-7)
- **Day 6**: Anchor Setup & Project Structure
- **Day 7**: Anchor Data & Constraints

### Phase 4: Building Simple Solana Programs (Days 8-9)
- **Day 8**: Counter Program Implementation
- **Day 9**: Deployment & Client Interaction

### Phase 5: Advanced Topics (Days 10-12)
- **Day 10**: Security & Upgradability
- **Day 11**: Cross-Program Invocations & PDAs
- **Day 12**: Best Practices & Production Readiness

## 🚀 Getting Started

### Prerequisites
- Basic programming knowledge
- Familiarity with command line
- **Only Rust required for Days 1-3!**

### Minimal Setup (Start Here!)

1. **Install Rust** (Only requirement for now):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustc --version  # Verify installation
   ```

2. **Start Learning** (No heavy dependencies!):
   ```bash
   # Read the step-by-step guide
   cat GETTING-STARTED.md

   # Begin with Day 1
   cat docs/day01-guide.md
   ```

### Optional Setup (For Later Days)

**Install when you reach Day 4+:**
- **Solana CLI**: `sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"`
- **Node.js**: For Anchor framework (Day 6+)
- **Anchor CLI**: `npm install -g @coral-xyz/anchor-cli`

## 📁 Project Structure

```
rust_solana/
├── README.md
├── Cargo.toml                 # Main workspace configuration
├── src/
│   ├── main.rs               # Entry point with module imports
│   ├── day01/                # Day 1: Rust Basics
│   │   ├── morning/          # Morning lessons
│   │   └── afternoon/        # Afternoon lessons
│   ├── day02/                # Day 2: Control Flow & Data Structures
│   ├── day03/                # Day 3: Traits & Error Handling
│   ├── day04/                # Day 4: Solana Architecture
│   ├── day05/                # Day 5: Transactions & Programs
│   ├── day06/                # Day 6: Anchor Setup
│   ├── day07/                # Day 7: Anchor Data
│   ├── day08/                # Day 8: Counter Program
│   ├── day09/                # Day 9: Deployment
│   ├── day10/                # Day 10: Security
│   ├── day11/                # Day 11: CPIs & PDAs
│   └── day12/                # Day 12: Best Practices
├── programs/                 # Anchor programs (from Day 6+)
├── tests/                    # Test files
├── app/                      # Client-side TypeScript code
└── docs/                     # Additional documentation
```

## 🎓 How to Use This Course

1. **Daily Structure**: Each day is divided into morning and afternoon sessions
2. **Hands-on Learning**: Every concept includes practical exercises
3. **Progressive Building**: Each lesson builds upon previous knowledge
4. **Module System**: Uncomment modules in `src/main.rs` as you progress
5. **Testing**: Run tests to verify your understanding

## 🔧 Running the Code

### Step-by-Step Approach (Recommended)

```bash
# 1. Start with reading and understanding (no compilation needed!)
cat GETTING-STARTED.md
cat docs/day01-guide.md

# 2. Quick syntax check (very fast)
chmod +x scripts/test-day01.sh
./scripts/test-day01.sh

# 3. When ready to run Day 1 examples
cp day01-only.toml Cargo.toml  # Use minimal dependencies
cargo run --bin day01

# 4. For future days (add dependencies as needed)
# We'll update Cargo.toml progressively
```

### Traditional Approach (If You Prefer)
```bash
# Run all examples (may have heavy dependencies)
cargo run

# Run tests
cargo test

# For Anchor programs (Days 6+)
anchor build
anchor test
anchor deploy
```

## 📖 Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)

## 🤝 Learning Approach

- **Practice-First**: Write code for every concept
- **Incremental**: Master one concept before moving to the next
- **Real-World**: Build actual programs you can deploy
- **Community**: Join Solana Discord and forums for help

---

**Ready to start your journey?** Begin with Day 1 in the `src/day01/` directory!

## 🤝 Contributing

We welcome contributions to improve this learning resource! Here's how you can help:

- 🐛 **Report bugs** - Found an issue? Open an issue!
- 📝 **Improve documentation** - Help make explanations clearer
- ✨ **Add examples** - More practice exercises are always welcome
- 🔧 **Fix code** - Submit PRs for improvements
- 💡 **Suggest features** - Ideas for new learning modules

### Development Setup
```bash
git clone https://github.com/yourusername/rust_solana
cd rust_solana
rustc --version  # Ensure Rust is installed
./scripts/test-day01.sh  # Quick verification
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- The amazing Rust and Solana communities

## ⭐ Star History

If this project helped you learn Rust and Solana, please consider giving it a star! ⭐

---

**Happy Learning!** 🦀🚀
