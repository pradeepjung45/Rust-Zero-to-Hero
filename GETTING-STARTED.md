# 🚀 Getting Started: Step-by-Step Learning

Welcome to your Rust & Solana learning journey! This guide will help you start learning **without heavy dependencies or long compilation times**.

## 🎯 Learning Philosophy

- **Step-by-step progression** - Master one concept before moving to the next
- **Lightweight approach** - No heavy dependencies until you need them
- **Hands-on learning** - Read, understand, then practice
- **No rushing** - Take time to understand each concept thoroughly

## 📋 Day 1: Getting Started (No Compilation Needed!)

### Step 1: Read and Understand
1. **Start here**: Open `docs/day01-guide.md` and read the concepts
2. **Study the code**: Look at `src/day01/morning/mod.rs` for basic syntax
3. **Understand ownership**: Read `src/day01/afternoon/mod.rs` for ownership concepts

### Step 2: Practice Without Running
- **Modify examples**: Change values, add your own variables
- **Predict outcomes**: Think about what each code snippet will do
- **Understand errors**: Read the commented-out lines that would cause errors

### Step 3: Quick Syntax Check (Optional)
```bash
# Make the test script executable
chmod +x scripts/test-day01.sh

# Run syntax check (very fast, no compilation)
./scripts/test-day01.sh
```

### Step 4: When Ready to Run Code
If you want to actually run the Day 1 examples:

```bash
# Option 1: Use the minimal configuration
cp day01-only.toml Cargo.toml
cargo run --bin day01

# Option 2: Or just check syntax without running
rustc --crate-type lib src/day01/morning/mod.rs --emit=metadata -o /tmp/check
```

## 📚 Day 1 Learning Focus

### Morning Session: Core Syntax & Variables
- **Variables**: `let`, `mut`, shadowing
- **Data Types**: integers, floats, booleans, characters
- **Compound Types**: tuples, arrays
- **Type System**: inference vs explicit types

### Afternoon Session: Ownership & Borrowing
- **Ownership Rules**: single owner, scope-based cleanup
- **Move Semantics**: transferring ownership
- **Borrowing**: `&` (immutable) and `&mut` (mutable) references
- **Lifetimes**: ensuring references remain valid

## 🔍 Key Day 1 Examples to Study

### 1. Basic Variables
```rust
let x = 5;           // Immutable
let mut y = 10;      // Mutable
let z = 20;
let z = z + 5;       // Shadowing
```

### 2. Ownership in Action
```rust
let s1 = String::from("hello");
let s2 = s1;         // s1 is moved, no longer valid
// println!("{}", s1); // Would cause error!
```

### 3. Borrowing
```rust
let s = String::from("hello");
let len = calculate_length(&s);  // Borrow s
println!("{} has length {}", s, len);  // s still valid!
```

## 🎯 Success Criteria for Day 1

Before moving to Day 2, make sure you can:

- [ ] Explain the difference between `let` and `let mut`
- [ ] Understand when values are moved vs copied
- [ ] Know when to use `&` vs `&mut`
- [ ] Explain the three ownership rules
- [ ] Predict when the compiler will give ownership errors

## 🔄 Next Steps

1. **Master Day 1 concepts** - Don't rush!
2. **When ready**: We'll add Day 2 with control flow and data structures
3. **Gradual progression**: Each day builds on the previous
4. **Add dependencies as needed**: We'll only add what we actually use

## 💡 Tips for Success

- **Read first, code second** - Understand before implementing
- **Experiment freely** - Modify examples and see what happens
- **Use the compiler as a teacher** - Rust error messages are very helpful
- **Take breaks** - Let concepts sink in
- **Ask questions** - Don't hesitate to ask for clarification

## 🛠️ Troubleshooting

### If you see compilation errors:
1. Make sure you're using the right Cargo.toml
2. Check that you haven't uncommented dependencies you don't need yet
3. Use the syntax check script instead of full compilation

### If examples don't make sense:
1. Re-read the guide in `docs/day01-guide.md`
2. Focus on one concept at a time
3. Try the practice exercises at the bottom of each module

---

**Ready to start?** Open `docs/day01-guide.md` and begin your Rust journey! 🦀
