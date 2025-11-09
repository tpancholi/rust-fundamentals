fn process_numbers_safe(slice: &[i32]) -> Result<(), String> {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            return Err(format!("Negative number found at index {}", index));
        }
    }
    Ok(())
}

fn main() {
    let test_cases = [
        vec![1, 2, 3, 4],      // Valid
        vec![5, -2, 3],        // Invalid - negative number
        vec![10, 20, 30]
    ];

    for (i, numbers) in test_cases.iter().enumerate() {
        println!("Test case {}: {:?}", i + 1, numbers);

        match process_numbers_safe(numbers) {
            Ok(()) => println!("  ✓ All numbers are positive!"),
            Err(msg) => println!("  ✗ Error: {}", msg),
        }
        println!();
    }
}