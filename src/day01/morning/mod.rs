// Day 1 Morning: Core Syntax & Variables
// Concepts: let, mut, data types, tuples, arrays, type inference

pub fn run_morning_examples() {
    println!("🌅 Morning Lesson: Core Syntax & Variables");
    println!("-----------------------------------------");
    
    // Uncomment each section as you learn:
    
    basic_variables();
    data_types_demo();
    tuples_and_arrays();
    type_inference_vs_explicit();

    practice_exercises();
    
    println!("\n✅ Morning exercises completed!");
    println!("💡 Practice: Try modifying the values and see what happens!");
}

fn basic_variables() {
    println!("\n📝 1. Basic Variables:");
    
    // Immutable by default
    let x = 5;
    println!("   Immutable x: {}", x);
    
    // Mutable variables
    let mut y = 10;
    println!("   Mutable y before: {}", y);
    y = 15;
    println!("   Mutable y after: {}", y);
    
    // Variable shadowing
    let z = 20;
    let z = z + 5;  // This shadows the previous z
    println!("   Shadowed z: {}", z);
}

fn data_types_demo() {
    println!("\n🔢 2. Data Types:");
    
    // Integers
    let small_int: i8 = 127;
    let big_int: i64 = 1_000_000;
    let unsigned: u32 = 42;
    
    // Floats
    let pi: f64 = 3.14159;
    let simple_float = 2.5; // f64 by default
    
    // Boolean
    let is_rust_awesome = true;
    let is_learning = false;
    
    // Character (Unicode scalar value)
    let letter = 'R';
    let emoji = '🦀';
    
    println!("   Integer i8: {}", small_int);
    println!("   Integer i64: {}", big_int);
    println!("   Unsigned u32: {}", unsigned);
    println!("   Float f64: {}", pi);
    println!("   Float (inferred): {}", simple_float);
    println!("   Boolean: {}", is_rust_awesome);
    println!("   Boolean: {}", is_learning);
    println!("   Character: {}", letter);
    println!("   Emoji: {}", emoji);
}

fn tuples_and_arrays() {
    println!("\n📦 3. Tuples and Arrays:");
    
    // Tuples - can hold different types
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);
    let (name, age, is_active) = person; // Destructuring
    println!("   Tuple: name={}, age={}, active={}", name, age, is_active);
    
    // Accessing tuple elements by index
    let coordinates = (10.5, 20.3);
    println!("   Coordinates: x={}, y={}", coordinates.0, coordinates.1);
    
    // Arrays - same type, fixed size
    let numbers = [1, 2, 3, 4, 5];
    let first = numbers[0];
    let length = numbers.len();
    println!("   Array: {:?}", numbers);
    println!("   First element: {}, Length: {}", first, length);
    
    // Array with explicit type and size
    let zeros: [i32; 3] = [0; 3]; // [0, 0, 0]
    println!("   Zeros array: {:?}", zeros);
}

fn type_inference_vs_explicit() {
    println!("\n🔍 4. Type Inference vs Explicit Types:");
    
    // Type inference - Rust figures out the type
    let inferred = 42;        // i32 by default
    let inferred_float = 3.14; // f64 by default
    
    // Explicit types
    let explicit: u16 = 42;
    let explicit_float: f32 = 3.14;
    
    println!("   Inferred integer: {} (i32)", inferred);
    println!("   Explicit integer: {} (u16)", explicit);
    println!("   Inferred float: {} (f64)", inferred_float);
    println!("   Explicit float: {} (f32)", explicit_float);
    
    // Sometimes you need to help the compiler
    let parsed: i32 = "42".parse().expect("Not a number!");
    println!("   Parsed string to i32: {}", parsed);
}

fn practice_exercises() {
    println!("\n🎯 Practice Exercises:");

    // 1. Create variables of different types and print them
    println!("\n   Exercise 1: Variables of different types");
    let an_integer = 100;         // Rust infers this as i32
    let a_float = 3.14;           // Rust infers this as f64
    let a_boolean = true;         // A simple boolean
    let a_character = 'Z';          // A single character
    println!("     Integer: {}, Float: {}, Boolean: {}, Char: {}", an_integer, a_float, a_boolean, a_character);
    println!("     Explanation: We declare variables with `let`. Rust infers their types from the values assigned.");

    // 2. Try to modify an immutable variable
    println!("\n   Exercise 2: Immutable variable");
    let immutable_var = 50;
    println!("     Initial value: {}", immutable_var);
    // immutable_var = 60; // This line is commented out because it will cause a compiler error!
    println!("     Explanation: Variables declared with `let` are immutable by default. You cannot reassign them. To make them mutable, you must use `let mut`.");

    // 3. Create a tuple with your personal info and destructure it
    println!("\n   Exercise 3: Tuple destructuring");
    let personal_info: (&str, i32, &str) = ("Alex", 32, "USA");
    let (name, age, country) = personal_info; // This is destructuring
    println!("     My name is {}, I am {} and from {}.", name, age, country);
    println!("     Explanation: A tuple is a fixed-size collection of values of different types. We can extract the values into individual variables in a single step, which is called 'destructuring'.");

    // 4. Create an array of your favorite numbers and access elements
    println!("\n   Exercise 4: Array and element access");
    let favorite_numbers = [7, 11, 21, 42, 88];
    let third_favorite = favorite_numbers[2]; // Arrays are 0-indexed, so index 2 is the third element.
    println!("     My favorite numbers are: {:?}", favorite_numbers);
    println!("     My third favorite number is: {}", third_favorite);
    println!("     Explanation: An array holds a fixed-size list of elements of the same type. We access elements using their index in square brackets, starting from zero.");

    // 5. Practice type annotations with different numeric types
    println!("\n   Exercise 5: Type annotations");
    let small_signed: i8 = -10;
    let large_unsigned: u64 = 1_000_000_000_000;
    let specific_float: f32 = 1.618;
    println!("     i8: {}, u64: {}, f32: {}", small_signed, large_unsigned, specific_float);
    println!("     Explanation: While Rust has great type inference, we can be explicit about the type we want by adding a type annotation (e.g., `: i8`). This is useful for clarity or when the compiler needs help.");
}
