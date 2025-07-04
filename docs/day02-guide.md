# Day 2: Control Flow & Data Structures

## 🎯 Learning Objectives
By the end of Day 2, you will understand:
- Control flow with if/else, loops, and match expressions
- Function definition and usage
- Structs for custom data types
- Enums for representing choices
- Option<T> and Result<T, E> for safe programming
- Collections: Vec<T> and HashMap<K, V>

## 📅 Morning Session: Control Flow & Functions

### Concepts Covered
- **If/Else Expressions**: Conditional logic and branching
- **Loops**: `loop`, `while`, and `for` iterations
- **Match Expressions**: Pattern matching for complex conditions
- **Functions**: Parameters, return values, and expressions vs statements

### Key Takeaways
1. **`if` is an expression** - it can return values
2. **Loops can return values** using `break` with a value
3. **Match is exhaustive** - must handle all possible cases
4. **Functions use expressions** for return values (no semicolon)

### Practice Examples
```rust
// If expression returning a value
let number = if condition { 5 } else { 6 };

// Loop returning a value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};

// Match with multiple patterns
match number {
    1 => println!("One"),
    2 | 3 | 5 | 7 => println!("Prime"),
    13..=19 => println!("Teen"),
    _ => println!("Other"),
}

// Function with return value
fn add(x: i32, y: i32) -> i32 {
    x + y  // Expression (no semicolon)
}
```

## 🌅 Afternoon Session: Structs, Enums & Collections

### Concepts Covered
- **Structs**: Custom data types with named fields
- **Enums**: Types that can be one of several variants
- **Option<T>**: Handling nullable values safely
- **Result<T, E>**: Error handling without exceptions
- **Vec<T>**: Dynamic arrays
- **HashMap<K, V>**: Key-value storage

### Key Takeaways
1. **Structs group related data** with meaningful names
2. **Enums represent choices** and can hold data
3. **Option<T> eliminates null pointer errors**
4. **Result<T, E> makes error handling explicit**
5. **Collections are growable** and type-safe

### Practice Examples
```rust
// Struct definition and usage
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

// Option handling
let some_value = Some(5);
match some_value {
    Some(x) => println!("Value: {}", x),
    None => println!("No value"),
}

// Vector operations
let mut v = vec![1, 2, 3];
v.push(4);
for item in &v {
    println!("{}", item);
}

// HashMap usage
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

## 🚀 Running the Code

### Option 1: Run Day 2 specifically
```bash
cargo run --bin day02
```

### Option 2: Run from main (both Day 1 and Day 2)
```bash
cargo run
```

### Option 3: Run tests
```bash
cargo test day02
```

## 🔍 Common Patterns to Learn

### 1. **Safe Array Access**
```rust
let v = vec![1, 2, 3];
match v.get(2) {
    Some(value) => println!("Found: {}", value),
    None => println!("Index out of bounds"),
}
```

### 2. **Error Handling**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### 3. **Enum Pattern Matching**
```rust
match message {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text: {}", text),
}
```

## 💡 Tips for Success

1. **Practice pattern matching** - it's fundamental to Rust
2. **Use Option and Result** instead of panicking
3. **Prefer iterators** over manual indexing
4. **Use `#[derive(Debug)]`** to print structs and enums
5. **Start with Vec and HashMap** for collections

## 📚 Additional Resources

- [The Rust Book - Chapter 3: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Rust Book - Chapter 5: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book - Chapter 6: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Chapter 8: Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

## ✅ Day 2 Checklist

- [ ] Understand if/else expressions and their return values
- [ ] Know the difference between loop, while, and for
- [ ] Can use match expressions with multiple patterns
- [ ] Can define and call functions with parameters and return values
- [ ] Understand struct definition and instantiation
- [ ] Know how to use enums with data
- [ ] Can handle Option<T> with match or if let
- [ ] Understand Result<T, E> for error handling
- [ ] Can create and manipulate vectors
- [ ] Can use HashMaps for key-value storage
- [ ] Have run all Day 2 examples successfully

**Ready for Day 3?** Tomorrow we'll explore traits, generics, and advanced error handling!
