// Day 2 Morning: Control Flow & Functions
// Concepts: if/else, loops (loop, while, for), match, functions

pub fn run_morning_examples() {
    println!("🌅 Morning Lesson: Control Flow & Functions");
    println!("-------------------------------------------");

    // Uncomment each section as you learn:

    if_else_expressions();
    loops_and_iteration();
    match_expressions();
    functions_examples();

    run_practice_exercises();

    println!("\n✅ Morning exercises completed!");
    println!("💡 Practice: Try modifying the conditions and see what happens!");
}

fn if_else_expressions() {
    println!("\n🔍 1. If/Else Expressions:");
    
    let number = 7;

    if number < 5 {
        println!("   Condition was true");
    } else {
        println!("   Condition was false");
    }

    // `if` is an expression, so you can use it in `let` statements.
    // The types of each block must be the same!
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("   The value from the if expression is: {}", value);

    // Example with else if
    let n = 6;
    if n % 4 == 0 {
        println!("   n is divisible by 4");
    } else if n % 3 == 0 {
        println!("   n is divisible by 3");
    } else if n % 2 == 0 {
        println!("   n is divisible by 2");
    } else {
        println!("   n is not divisible by 4, 3, or 2");
    }
    println!("   Explanation: 'if/else' lets you branch your code based on conditions. Unlike many languages, 'if' is an expression in Rust, meaning it returns a value.");
}

fn loops_and_iteration() {
    println!("\n🔄 2. Loops and Iteration:");

    // loop - infinite loop until break
    println!("   Loop example:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;  // loop can return a value
        }
    };
    println!("   Loop result: {}", result);

    // while loop
    println!("   While loop example:");
    let mut number = 3;
    while number != 0 {
        println!("   {}!", number);
        number -= 1;
    }
    println!("   LIFTOFF!!!");

    // for loop with range
    println!("   For loop with range:");
    for number in 1..4 {
        println!("   Number: {}", number);
    }

    // for loop with array
    println!("   For loop with array:");
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("   Element: {}", element);
    }

    // for loop with iterator methods
    println!("   For loop with iterator (reversed):");
    for number in (1..4).rev() {
        println!("   Countdown: {}", number);
    }
}

fn match_expressions() {
    println!("\n🎯 3. Match Expressions:");

    let number = 13;
    println!("   Tell me about {}", number);

    match number {
        1 => println!("   One!"),
        2 | 3 | 5 | 7 | 11 => println!("   This is a prime"),
        13..=19 => println!("   A teen"),
        _ => println!("   Ain't special"),
    }

    // Match with boolean
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("   Boolean {} -> binary {}", boolean, binary);

    // Match with tuples
    let pair = (0, -2);
    println!("   Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("   First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("   `x` is `{:?}` and last is `0`", x),
        _ => println!("   It doesn't matter what they are"),
    }
}

fn functions_examples() {
    println!("\n🔧 4. Functions:");

    // Function with parameters and return value
    let result = add_numbers(5, 3);
    println!("   5 + 3 = {}", result);

    // Function with multiple parameters
    let area = calculate_area(4.0, 5.0);
    println!("   Area of 4.0 x 5.0 rectangle: {}", area);

    // Function that returns early
    let grade = get_letter_grade(85);
    println!("   Grade for 85: {}", grade);

    // Function with no return value (returns unit type ())
    print_separator();
}

// Function definitions
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // No semicolon = expression (returns value)
}

fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn get_letter_grade(score: u32) -> char {
    if score >= 90 {
        return 'A';  // Early return with semicolon
    }

    match score {
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn print_separator() {
    println!("   ========================");
}

// 🎯 Practice Exercises:
// Solves the practice exercises from the lesson.
fn run_practice_exercises() {
    println!("\n🎯 Practice Exercises:");

    // 1. Write a function that checks if a number is even or odd
    fn is_even_or_odd(n: i32) {
        if n % 2 == 0 {
            println!("   1. {} is even.", n);
        } else {
            println!("   1. {} is odd.", n);
        }
    }
    is_even_or_odd(10);
    is_even_or_odd(7);

    // 2. Create a loop that prints the first 10 Fibonacci numbers
    fn print_first_10_fibonacci() {
        let mut a = 0;
        let mut b = 1;
        print!("   2. First 10 Fibonacci numbers: ");
        for _ in 0..10 {
            print!("{} ", a);
            let next = a + b;
            a = b;
            b = next;
        }
        println!();
    }
    print_first_10_fibonacci();

    // 3. Use match to create a simple calculator (add, subtract, multiply, divide)
    enum Operation { Add, Subtract, Multiply, Divide }
    fn simple_calculator(op: Operation, x: f64, y: f64) -> f64 {
        match op {
            Operation::Add => x + y,
            Operation::Subtract => x - y,
            Operation::Multiply => x * y,
            Operation::Divide => x / y,
        }
    }
    let add_result = simple_calculator(Operation::Add, 10.0, 5.0);
    let div_result = simple_calculator(Operation::Divide, 20.0, 4.0);
    println!("   3. Calculator: 10 + 5 = {}, 20 / 4 = {}", add_result, div_result);

    // 4. Write a function that finds the largest number in an array using a for loop
    fn find_largest(arr: &[i32]) -> i32 {
        // Note: In real Rust, you'd just use arr.iter().max().unwrap()
        let mut largest = arr[0];
        for &item in arr.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let my_array = [1, 5, 2, 9, 3];
    let largest_num = find_largest(&my_array);
    println!("   4. The largest number in {:?} is {}.", my_array, largest_num);


    // 5. Create a guessing game using loop, if/else, and match
    fn guessing_game() {
        let secret_number = 7; // In a real game, this would be random.
        let mut guess: i32 = 5; // We'll simulate a starting guess.

        println!("   5. Guessing Game (simulation):");
        println!("      I'm thinking of a number. Let's find it.");

        loop {
             println!("      My guess is {}.", guess);
            match guess.cmp(&secret_number) {
                std::cmp::Ordering::Less => {
                    println!("      Too small! Trying a larger number.");
                    guess += 2; // Simulate another guess
                },
                std::cmp::Ordering::Greater => {
                    println!("      Too big! Trying a smaller number.");
                    guess -= 2; // Simulate another guess
                },
                std::cmp::Ordering::Equal => {
                    println!("      You got it! The number was {}.", secret_number);
                    break;
                }
            }
        }
    }
    guessing_game();
}