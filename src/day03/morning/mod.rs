// Day 3 Morning: Traits & Generics
// Concepts: Defining and implementing traits, trait objects, generics for functions and structs

pub fn run_morning_examples() {
    println!("🌅 Morning Lesson: Traits & Generics");
    println!("------------------------------------");

    // Uncomment each section as you learn:
    
    traits_basics();
    trait_implementations();
    generic_functions();
    generic_structs();
    trait_bounds();
    
    println!("\n✅ Morning exercises completed!");
    println!("💡 Practice: Try implementing common traits like Display and Debug!");
}

fn traits_basics() {
    println!("\n🎭 1. Traits Basics:");
    
    // Define a trait
    trait Summary {
        fn summarize(&self) -> String;
        
        // Default implementation
        fn summarize_author(&self) -> String {
            String::from("(Read more...)")
        }
    }
    
    // Implement trait for a struct
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let article = NewsArticle {
        headline: String::from("Rust is Amazing!"),
        location: String::from("San Francisco"),
        author: String::from("Alice"),
        content: String::from("Rust provides memory safety without garbage collection..."),
    };
    
    let tweet = Tweet {
        username: String::from("bob_dev"),
        content: String::from("Learning Rust traits today! #rustlang"),
        reply: false,
        retweet: false,
    };
    
    println!("   Article: {}", article.summarize());
    println!("   Article author: {}", article.summarize_author());
    println!("   Tweet: {}", tweet.summarize());
    println!("   Tweet author: {}", tweet.summarize_author()); // Uses default implementation
}

fn trait_implementations() {
    println!("\n🔧 2. Common Trait Implementations:");
    
    // Display trait for custom formatting
    use std::fmt;
    
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1.clone();
    
    println!("   Point with Display: {}", p1);
    println!("   Point with Debug: {:?}", p1);
    println!("   Points equal: {}", p1 == p2);
    println!("   Cloned point: {}", p3);
    
    // Iterator trait
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    println!("   Custom iterator:");
    let mut counter = Counter::new(3);
    for num in counter {
        println!("     Count: {}", num);
    }
}

fn generic_functions() {
    println!("\n🧬 3. Generic Functions:");
    
    // Generic function to find the largest element in a slice
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let numbers = [34, 50, 25, 100, 65];
    let chars = ['y', 'm', 'a', 'q'];
    println!("   Largest number: {}", largest(&numbers));
    println!("   Largest char: {}", largest(&chars));
}

fn generic_structs() {
    println!("\n📦 4. Generic Structs:");
    
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("   Integer point: {:?}", int_point);
    println!("   Float point: {:?}", float_point);
}

fn trait_bounds() {
    println!("\n🔒 5. Trait Bounds:");
    
    // Function that prints any type that implements Display
    use std::fmt::Display;
    fn print_item<T: Display>(item: T) {
        println!("   Item: {}", item);
    }
    print_item(42);
    print_item("hello");
    
    // Multiple trait bounds
    fn compare_and_display<T: Display + PartialOrd>(a: T, b: T) {
        if a > b {
            println!("   {} is greater than {}", a, b);
        } else {
            println!("   {} is not greater than {}", a, b);
        }
    }
    compare_and_display(10, 5);
    compare_and_display(3.14, 2.71);
}

// Practice Exercises
fn practice_exercises() {
    println!("\n🎯 Practice Exercises:");
    // 1. Implement a trait for a custom struct
    trait Greet {
        fn greet(&self) -> String;
    }
    struct Person {
        name: String,
    }
    impl Greet for Person {
        fn greet(&self) -> String {
            format!("Hello, {}!", self.name)
        }
    }
    let p = Person { name: String::from("Alice") };
    println!("   {}", p.greet());
    // 2. Write a generic function that returns the sum of two values
    fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    println!("   Sum of 3 + 4: {}", sum(3, 4));
    println!("   Sum of 1.5 + 2.5: {}", sum(1.5, 2.5));
    // 3. Use trait bounds to write a function that prints the debug representation of any value
    fn print_debug<T: std::fmt::Debug>(item: T) {
        println!("   Debug: {:?}", item);
    }
    print_debug(vec![1, 2, 3]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest() {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let numbers = [3, 7, 2, 9];
        assert_eq!(largest(&numbers), 9);
    }
    #[test]
    fn test_greet_trait() {
        trait Greet { fn greet(&self) -> String; }
        struct Person { name: String }
        impl Greet for Person {
            fn greet(&self) -> String { format!("Hello, {}!", self.name) }
        }
        let p = Person { name: String::from("Test") };
        assert_eq!(p.greet(), "Hello, Test!");
    }
}
