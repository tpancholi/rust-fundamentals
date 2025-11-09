fn main() {
    for i in 0..=10 {
        if i % 2 == 0 {
            continue;
            // skip even numbers
        }
        println!("{}", i);
        if i == 7 {
            break;
            // exit loop at 7
        }
    }
}