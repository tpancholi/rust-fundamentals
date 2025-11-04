use std::io;

// using `loop` keyword for looping,
// it saves effort to create initial condition for looping
// warning - can lead to infinite loop easily if no break condition usually defined
//  helpful when the end loop condition is not very clear
fn main() {
    let mut x = 1;
    println!("========loop keyword===========");
    loop {
        println!("The value of x is {}", x);
        x += 1;
        // below is the loop end condition, very important
        if x > 5 {
            break;
        }
    }
    println!("Keyword based loop ended...");

    // while loop
    // usually used in the CLI app where user input is asked
    println!("\n========while loop===========");
    // example 1
    let mut i = 0;
    while i < 5 {
        println!("The value of i is {}", i);
        i += 1;
    }
    // example 2
    let mut user_input = String::new();
    while user_input.trim() != "qwertyuiop" {
        user_input.clear();
        println!("Please input a user input. (type \"qwertyuiop\" to stop): ");
        io::stdin().read_line(&mut user_input).expect("Failed to read input from user");
        println!("You wrote: {}", user_input);
    }
    println!("While loop ended...");
    // for loop
    // the loop which runs based on a pre-determined range (e.g. `(1..10)` or `(1..=10)`
    println!("=======for loop===========");

    //example 1
    for j in 1..5 {
        println!("The value of j is {}", j);
    }

    // example 2
    // below is the same as `1..=5`
    for rev_j in (1..=5).rev() {
        println!("The value of rev_j is {}", rev_j);
    }

    // example 3
    let tmp_array = ["a", "b", "c", "d", "e"];
    for item in tmp_array {
        println!("The value of item is {}", item);
    }

    // example 4
    let tmp_vec = vec![1, 2, 3, 4, 5];
    for num in tmp_vec {
        println!("The value of num is {}", num);
    }
    println!("for loop ended...");
}