# Day 1: Rust Basics - Syntax & Ownership

## 🎯 Learning Objectives
By the end of Day 1, you will understand:
- Rust's basic syntax and variable declarations
- Core data types and their usage
- Ownership, borrowing, and move semantics
- Basic lifetime concepts

## 📅 Morning Session: Core Syntax & Variables

### Concepts Covered
- **Variables**: `let`, `mut`, shadowing
- **Data Types**: integers, floats, booleans, characters
- **Compound Types**: tuples, arrays
- **Type Inference**: when Rust figures out types vs. explicit annotations

### Key Takeaways
1. **Variables are immutable by default** - use `mut` for mutability
2. **Shadowing allows reusing variable names** with potentially different types
3. **Rust has strong type safety** but excellent type inference
4. **Tuples can hold different types**, arrays hold same types

### Practice Exercises
```rust
// 1. Variable practice
let name = "Alice";
let mut age = 25;
age += 1;

// 2. Type annotations
let explicit_number: i32 = 42;
let inferred_number = 42; // Also i32

// 3. Tuples and destructuring
let person = ("Bob", 30, true);
let (name, age, active) = person;

// 4. Arrays
let numbers = [1, 2, 3, 4, 5];
let first = numbers[0];
```

## 🌅 Afternoon Session: Ownership, Borrowing & Lifetimes

### Concepts Covered
- **Ownership Rules**: single owner, scope-based cleanup
- **Move Semantics**: transferring ownership
- **Borrowing**: immutable (`&`) and mutable (`&mut`) references
- **Lifetimes**: ensuring references remain valid

### Key Takeaways
1. **Each value has exactly one owner** at any time
2. **Moving transfers ownership** and invalidates the previous owner
3. **Borrowing allows using values without taking ownership**
4. **Only one mutable borrow OR multiple immutable borrows** at a time
5. **Lifetimes prevent dangling references**

### Practice Exercises
```rust
// 1. Ownership and moves
let s1 = String::from("hello");
let s2 = s1; // s1 is moved, no longer valid

// 2. Borrowing
let s3 = String::from("world");
let len = calculate_length(&s3); // s3 is borrowed, still valid

// 3. Mutable borrowing
let mut s4 = String::from("hello");
change(&mut s4);

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

## 🚀 Running the Code

### Option 1: Run Day 1 specifically
```bash
cargo run --bin day01
```

### Option 2: Run from main (after uncommenting)
1. In `src/main.rs`, uncomment:
   ```rust
   mod day01;
   // and in main():
   day01::run_examples();
   ```
2. Run: `cargo run`

### Option 3: Run tests
```bash
cargo test day01
```

## 🔍 Common Beginner Mistakes

1. **Trying to use moved values**:
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{}", s1); // ❌ Error: s1 was moved
   ```

2. **Multiple mutable borrows**:
   ```rust
   let mut s = String::from("hello");
   let r1 = &mut s;
   let r2 = &mut s; // ❌ Error: cannot borrow as mutable more than once
   ```

3. **Mixing immutable and mutable borrows**:
   ```rust
   let mut s = String::from("hello");
   let r1 = &s;     // immutable borrow
   let r2 = &mut s; // ❌ Error: cannot borrow as mutable
   ```

## 💡 Tips for Success

1. **Read compiler errors carefully** - Rust's error messages are very helpful
2. **Start with immutable by default** - only add `mut` when needed
3. **Use borrowing instead of moving** when you want to keep using a value
4. **Practice with the examples** - modify them and see what breaks
5. **Don't worry about lifetimes yet** - focus on ownership and borrowing first

## 📚 Additional Resources

- [The Rust Book - Chapter 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [The Rust Book - Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Variables](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)

## ✅ Day 1 Checklist

- [ ] Understand variable declarations and mutability
- [ ] Know the basic data types (integers, floats, booleans, chars)
- [ ] Can work with tuples and arrays
- [ ] Understand the three ownership rules
- [ ] Can explain the difference between moving and borrowing
- [ ] Know when to use `&` vs `&mut`
- [ ] Have run all the Day 1 examples successfully

**Ready for Day 2?** Tomorrow we'll dive into control flow and more complex data structures!
