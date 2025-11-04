fn main() {
    let mut height = 190;
    height -= 20;


    // semicolon is missing bcoz if condition becomes true value will assign to result (e.g. "Tall")
    let result = if height > 180 {
        "Tall"
    } else if height <= 180 && height > 160 {
        "Average"
    } else {
        "Short"
    };
    println!("Your height is {}", result);

    // inline condition-based assignment
    let health = if height < 180 { "good" } else { "bad" };
    println!("The health is {}", health);

    // shadowing variable with different type with boolean in place of string
    let health = height > 180;
    println!("The height is {}", health);
    // the above is allowed bcoz of "let" we are basically creating a new variable,
    // if you remove that then rust will throw error of mismatch type between `&str` and `bool`
    // this should only be used when we have a strong use case else please avoid it
}