// Day 2 Afternoon: Structs, Enums & Collections
// Concepts: Structs, Enums, Option<T>, Result<T, E>, Vec<T>, HashMap<K, V>

use std::collections::HashMap;

pub fn run_afternoon_examples() {
    println!("🌅 Afternoon Lesson: Structs, Enums & Collections");
    println!("-------------------------------------------------");

    // Uncomment each section as you learn:

    structs_examples();
    enums_examples();
    option_and_result();
    vectors_examples();
    hashmaps_examples();

    println!("\n✅ Afternoon exercises completed!");
    println!("💡 Practice: Try creating your own structs and enums!");
}

fn structs_examples() {
    println!("\n🏗️ 1. Structs:");

    // Define a struct
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Create an instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 3,
    };

    println!("   User: {:?}", user1);
    println!("   Username: {}", user1.username);

    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    println!("   Updated user email: {}", user2.email);

    // Struct update syntax
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirduser"),
        ..user1  // Use remaining fields from user1
    };

    println!("   User3 sign-in count: {}", user3.sign_in_count);

    // Tuple structs
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("   Color: {:?}, Point: {:?}", black, origin);

    // Unit-like structs
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn enums_examples() {
    println!("\n🎭 2. Enums:");

    // Basic enum
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("   IP versions: {:?}, {:?}", four, six);

    // Enum with data
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("   Home: {:?}", home);
    println!("   Loopback: {:?}", loopback);

    // Enum with different types
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("   Messages: {:?}, {:?}, {:?}, {:?}", msg1, msg2, msg3, msg4);

    // Using match with enums
    match msg2 {
        Message::Quit => println!("   The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("   Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("   Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("   Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

fn option_and_result() {
    println!("\n🎁 3. Option<T> and Result<T, E>:");

    // Option<T> - represents a value that might be present or absent
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("   Some number: {:?}", some_number);
    println!("   Some string: {:?}", some_string);
    println!("   Absent number: {:?}", absent_number);

    // Using match with Option
    match some_number {
        Some(value) => println!("   Found a value: {}", value),
        None => println!("   No value found"),
    }

    // Using if let with Option
    if let Some(value) = some_string {
        println!("   String value: {}", value);
    }

    // Result<T, E> - represents success (Ok) or failure (Err)
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Ok(value) => println!("   Division result: {}", value),
        Err(error) => println!("   Error: {}", error),
    }

    match result2 {
        Ok(value) => println!("   Division result: {}", value),
        Err(error) => println!("   Error: {}", error),
    }

    // Using unwrap_or for default values
    let good_result = divide(8.0, 2.0).unwrap_or(0.0);
    let bad_result = divide(8.0, 0.0).unwrap_or(0.0);

    println!("   Good result with default: {}", good_result);
    println!("   Bad result with default: {}", bad_result);
}

fn vectors_examples() {
    println!("\n📚 4. Vectors (Vec<T>):");

    // Creating vectors
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];  // vec! macro

    println!("   Empty vector: {:?}", v1);
    println!("   Vector with values: {:?}", v2);

    // Adding elements
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("   After pushing: {:?}", v1);

    // Accessing elements
    let third: &i32 = &v2[2];  // Direct indexing (panics if out of bounds)
    println!("   Third element: {}", third);

    match v2.get(2) {  // Safe access with Option
        Some(third) => println!("   Third element (safe): {}", third),
        None => println!("   There is no third element."),
    }

    // Iterating over vectors
    println!("   Iterating over v2:");
    for i in &v2 {
        println!("     {}", i);
    }

    // Mutable iteration
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;  // Dereference to modify the value
    }
    println!("   After adding 50 to each: {:?}", v3);

    // Vectors with enums (storing different types)
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("   Spreadsheet row: {:?}", row);
}

fn hashmaps_examples() {
    println!("\n🗺️ 5. HashMaps:");

    // Creating a HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("   Scores: {:?}", scores);

    // Creating from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("   Scores from vectors: {:?}", scores2);

    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("   Blue team score: {}", s),
        None => println!("   Team not found"),
    }

    // Iterating over HashMap
    println!("   All scores:");
    for (key, value) in &scores {
        println!("     {}: {}", key, value);
    }

    // Updating values
    scores.insert(String::from("Blue"), 25);  // Overwrite
    println!("   After updating Blue: {:?}", scores);

    // Only insert if key doesn't exist
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(40);  // Won't overwrite
    println!("   After conditional inserts: {:?}", scores);

    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("   Word count: {:?}", map);
}

// 🎯 Practice Exercises:
// 1. Create a struct for a Book with title, author, and pages
// 2. Create an enum for different types of vehicles (Car, Bike, Plane) with associated data
// 3. Write a function that takes a vector of integers and returns the mean, median, and mode
// 4. Create a HashMap that stores employee names and their departments
// 5. Use Option<T> to create a function that safely gets the first element of a vector
// 6. Create a Result<T, E> function that parses a string to an integer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
    }

    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("key2"), None);
    }

    #[test]
    fn test_option_handling() {
        let some_value = Some(42);
        let none_value: Option<i32> = None;

        assert_eq!(some_value.unwrap_or(0), 42);
        assert_eq!(none_value.unwrap_or(0), 0);
    }
}
