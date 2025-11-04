fn main() {
    println!("=== Variables Example ===");

    // Immutable variable
    let x = 5;
    println!("Immutable x: {}", x);

    // Mutable variable
    let mut y = 10;
    y += 1;
    println!("Mutable y: {}", y);

    // Shadowing
    let z = 5;
    let z = z + 1;
    println!("Shadowed z: {}", z);
}
