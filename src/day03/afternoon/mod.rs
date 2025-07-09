// Day 3 Afternoon: Advanced Error Handling & Modules
// Concepts: Result, Option, custom errors, ? operator, error propagation, modules, pub, use

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

pub fn run_afternoon_examples() {
    println!("🌅 Afternoon Lesson: Advanced Error Handling & Modules");
    println!("------------------------------------------------------");

    error_handling_examples();
    custom_error_types();
    question_operator_and_propagation();
    modules_and_visibility();
    practice_exercises();

    println!("\n✅ Afternoon exercises completed!");
    println!("💡 Practice: Try writing your own error types and modules!");
}

fn error_handling_examples() {
    println!("\n❗ 1. Error Handling Basics:");
    // Option<T>
    let some = Some(10);
    let none: Option<i32> = None;
    println!("   Option Some: {:?}, None: {:?}", some, none);
    match some {
        Some(v) => println!("   Got value: {}", v),
        None => println!("   No value"),
    }
    // Result<T, E>
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("error!");
    println!("   Result Ok: {:?}, Err: {:?}", ok, err);
    match err {
        Ok(v) => println!("   Got value: {}", v),
        Err(e) => println!("   Error: {}", e),
    }
}

fn custom_error_types() {
    println!("\n🛠️ 2. Custom Error Types:");
    #[derive(Debug)]
    enum MyError {
        Io(io::Error),
        Parse(ParseIntError),
        Other(String),
    }
    // Example function that returns custom error
    fn double_number(input: &str) -> Result<i32, MyError> {
        let num: i32 = input.parse().map_err(MyError::Parse)?;
        Ok(num * 2)
    }
    match double_number("21") {
        Ok(n) => println!("   Double: {}", n),
        Err(e) => println!("   Error: {:?}", e),
    }
    match double_number("oops") {
        Ok(n) => println!("   Double: {}", n),
        Err(e) => println!("   Error: {:?}", e),
    }
}

fn question_operator_and_propagation() {
    println!("\n❓ 3. The ? Operator and Error Propagation:");
    // Function that propagates errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    match read_username_from_file() {
        Ok(content) => println!("   File content: {}", content),
        Err(e) => println!("   Error reading file: {}", e),
    }
}

fn modules_and_visibility() {
    println!("\n📦 4. Modules and Visibility:");
    mod outer {
        pub fn public_fn() {
            println!("   This is a public function in 'outer' module");
        }
        fn private_fn() {
            println!("   This is a private function in 'outer' module");
        }
        pub mod inner {
            pub fn inner_fn() {
                println!("   This is a public function in 'outer::inner' module");
            }
        }
    }
    outer::public_fn();
    // outer::private_fn(); // This would not compile
    outer::inner::inner_fn();
    use outer::public_fn as pf;
    pf();
}

fn practice_exercises() {
    println!("\n🎯 Practice Exercises:");
    // 1. Write a function that returns a Result and handles error with match
    fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }
    println!("   1. 10 / 2 = {:?}", safe_divide(10, 2));
    println!("   1. 10 / 0 = {:?}", safe_divide(10, 0));
    // 2. Use the ? operator in your own function
    fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
        let n: i32 = input.parse()?;
        Ok(n * 2)
    }
    println!("   2. '5' doubled = {:?}", parse_and_double("5"));
    println!("   2. 'oops' doubled = {:?}", parse_and_double("oops"));
    // 3. Create a module with a public function and call it
    mod greet {
        pub fn hello(name: &str) {
            println!("   Hello, {}!", name);
        }
    }
    greet::hello("Rustacean");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_safe_divide() {
        assert_eq!(super::practice_exercises::safe_divide(6, 2), Ok(3));
        assert_eq!(super::practice_exercises::safe_divide(1, 0), Err("Cannot divide by zero".to_string()));
    }
    #[test]
    fn test_parse_and_double() {
        assert_eq!(super::practice_exercises::parse_and_double("4"), Ok(8));
        assert!(super::practice_exercises::parse_and_double("err").is_err());
    }
} 