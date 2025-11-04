fn main() {
    println!("=== Data Types Example ===");

    // Integer types
    let integer: i32 = 42;
    let unsigned: u32 = 100;
    println!("Integer: {}, Unsigned: {}", integer, unsigned);

    // Floating point
    let float: f64 = std::f64::consts::PI;
    println!("Float: {}", float);

    // Boolean
    let is_rust_cool: bool = true;
    println!("Is Rust cool? {}", is_rust_cool);

    // Character
    let letter: char = 'R';
    println!("Letter: {}", letter);
}
