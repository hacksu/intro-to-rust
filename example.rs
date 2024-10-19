
fn main() {

    let mut x = 1;
    x = 0; // yay 

    for i in 0..10 {
        println!("{i}");
    }

    if x == 1 {
        println!("X is one!")
    } else if x == 2 {
        println!("X is two!")
    } else {
        println!("X is something else!")
    }

    while x != 0 {
        // ...
    }

    match x {
        1 => println!("X is one!"),
        2..5 => println!("X is 2 to 5!"),
        _ => println!("X is some other value!"),
    }

    let boolean = true;
    let output = match boolean {
        false => 0,
        true => 1,
    }
    
    println!("{output}");
}