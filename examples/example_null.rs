
fn main() {
    let mut val: Option<u64> = None; // value is "null"

    // this will never happen
    if let Some(number) = val {
        // ..?
    }

    val = Some(10);

    // this will happen!
    if let Some(number) = val {
        println!("the number is {}", number);
    }

    val = None;

    // we can even do this
    match val {
        Some(number) => println!("the number is {}", number),
        None => println!("rip, there is no number."),
    }
}