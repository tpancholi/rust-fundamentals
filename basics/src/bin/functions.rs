fn main() {
    println!("=== Functions Example ===");

    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);

    greet("Alice");
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // No semicolon = return value
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
