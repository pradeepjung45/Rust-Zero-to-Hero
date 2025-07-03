// Day 1 Afternoon: Ownership, Borrowing & Lifetimes
// Concepts: Ownership rules, move semantics, borrowing (&), mutable borrowing (&mut)

pub fn run_afternoon_examples() {
    println!("🌅 Afternoon Lesson: Ownership, Borrowing & Lifetimes");
    println!("----------------------------------------------------");
    
    // Uncomment each section as you learn:
    
    ownership_basics();
    move_semantics();
    borrowing_examples();
    mutable_borrowing();
    lifetime_introduction();

    practice_exercises();
    
    println!("\n✅ Afternoon exercises completed!");
    println!("💡 Practice: Try breaking the ownership rules and see compiler errors!");
}

fn ownership_basics() {
    println!("\n🏠 1. Ownership Basics:");
    println!("   Rust's Three Ownership Rules:");
    println!("   1. Each value has a single owner");
    println!("   2. When the owner goes out of scope, the value is dropped");
    println!("   3. There can only be one owner at a time");
    
    {
        let s = String::from("hello"); // s owns the string
        println!("   String s: {}", s);
    } // s goes out of scope and is dropped here
    
    // println!("{}", s); // This would cause a compile error!
    
    let x = 5;        // Copy types (like integers) are copied, not moved
    let y = x;        // x is still valid
    println!("   x: {}, y: {} (both valid - Copy trait)", x, y);
}

fn move_semantics() {
    println!("\n📦 2. Move Semantics:");
    
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    
    println!("   s2: {}", s2);
    // println!("{}", s1); // This would cause a compile error!
    
    // To keep both valid, we need to clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy
    println!("   s3: {}, s4: {} (both valid after clone)", s3, s4);
}

fn borrowing_examples() {
    println!("\n👀 3. Borrowing (Immutable References):");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1
    println!("   The length of '{}' is {}", s1, len); // s1 is still valid!
    
    // Multiple immutable borrows are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("   r1: {}, r2: {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is a reference, so we don't take ownership
} // s goes out of scope, but since it's just a reference, nothing happens

fn mutable_borrowing() {
    println!("\n✏️ 4. Mutable Borrowing:");
    
    let mut s = String::from("hello");
    change_string(&mut s); // Mutable borrow
    println!("   After change: {}", s);
    
    // Only one mutable borrow at a time!
    let r1 = &mut s;
    // let r2 = &mut s; // This would cause a compile error!
    println!("   Mutable reference r1: {}", r1);
    
    // Can't have immutable and mutable borrows simultaneously
    let s2 = String::from("world");
    let r3 = &s2;        // Immutable borrow
    // let r4 = &mut s2;    // This would cause a compile error!
    println!("   Immutable reference r3: {}", r3);
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

fn lifetime_introduction() {
    println!("\n⏰ 5. Lifetime Introduction:");
    println!("   Lifetimes ensure references are valid as long as needed");
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("   The longest string is: {}", result);s
    }
    // result is still valid here because it refers to string1
    // which is still in scope
}

// Lifetime annotation: 'a means both parameters and return value
// must live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn practice_exercises() {
    println!("\n🎯 Practice Exercises:");

    // 1. Create a function that takes ownership of a String and returns its length
    println!("\n   Exercise 1: Function takes ownership");
    let my_string = String::from("Owned by main");
    let len = takes_ownership_and_returns_length(my_string);
    println!("     Length is: {}", len);
    // println!("{}", my_string); // This would be an error because the function took ownership!
    println!("     Explanation: The `my_string` variable was moved into the function. After the function call, `my_string` is no longer valid in this scope.");

    // 2. Create a function that borrows a String and returns its length
    println!("\n   Exercise 2: Function borrows a String");
    let another_string = String::from("Borrowed by function");
    let len_borrowed = borrows_string_and_returns_length(&another_string);
    println!("     Length is: {}. Original string is still here: '{}'", len_borrowed, another_string);
    println!("     Explanation: We passed a reference (`&`) to the function. The function borrowed the value but did not take ownership, so `another_string` is still valid.");

    // 3. Try to create two mutable references to the same variable
    println!("\n   Exercise 3: Two mutable references (causes error)");
    let mut mutable_str = String::from("Can I change?");
    let _ref1 = &mut mutable_str;
    // let _ref2 = &mut mutable_str; // This line is commented out because it causes a compiler error!
    println!("     Explanation: Rust's borrowing rules state you can only have ONE mutable reference to a particular piece of data in a particular scope. This prevents data races.");

    // 4. Create a function that takes a mutable reference and modifies the string
    println!("\n   Exercise 4: Modifying through a mutable reference");
    let mut string_to_modify = String::from("Initial state");
    println!("     Before: {}", string_to_modify);
    add_suffix(&mut string_to_modify);
    println!("     After:  {}", string_to_modify);
    println!("     Explanation: We pass a mutable reference (`&mut`) to the function, allowing it to modify the original string directly.");

    // 5. Experiment with the lifetime of references in different scopes
    println!("\n   Exercise 5: Lifetimes and scopes");
    let outer_scope_str = "I live longer";
    let mut longest_str = "";
    {
        let inner_scope_str = "I am short-lived";
        longest_str = longest(outer_scope_str, inner_scope_str);
        println!("     Inside inner scope, longest is: '{}'", longest_str);
    }
    // longest_str = longest(outer_scope_str, inner_scope_str); // This would be an error because inner_scope_str does not live long enough!
    println!("     Explanation: The `longest` function's result has a lifetime tied to the SHORTEST of its inputs. We can use the result inside the inner scope, but once `inner_scope_str` is dropped, the reference would be invalid, and the compiler stops us.");
}

fn takes_ownership_and_returns_length(s: String) -> usize {
    s.len()
} // s is dropped here

fn borrows_string_and_returns_length(s: &String) -> usize {
    s.len()
} // s is not dropped, as it's just a reference

fn add_suffix(s: &mut String) {
    s.push_str(" - Modified!");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        // s is still valid here
        assert_eq!(s, "hello");
    }
    
    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "hi"), "hello");  // "hello" (5) > "hi" (2)
        assert_eq!(longest("short", "goodbye"), "goodbye");  // "goodbye" (7) > "short" (5)
    }
}
