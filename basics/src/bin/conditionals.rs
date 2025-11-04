fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding!!")
    } else {
        println!("Still waiting for confirmation ...")
    }
    let height = 170;
    if height > 180 {
        println!("You can consider yourself tall")
    } else if height <= 180 && height > 160 {
        println!("You can consider your self average")
    } else {
        println!("You are definitely short")
    }

    let maybenumber: Option<Option<()>> = Some(None);
    // let maybeNumber = Some(42);
    if let Some(number) = maybenumber {
        println!("The number is {:#?}", number);
    } else {
        println!("There is no number");
    }
}