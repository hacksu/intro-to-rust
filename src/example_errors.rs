

fn do_some_work(fail: bool) -> Result<String, String> {
    
    if fail {
        return Err(String::from("not yay!"));
    }

    Ok(String::from("yay!"))
}

fn main() {
    let passed = do_some_work(false);
    if let Ok(message) = passed {
        println!("{}", message)
    }

    let failed = do_some_work(true);
    if let Err(message) = failed {
        println!("{}", message)
    }

    let output = match do_some_work(true) {
        Ok(message) => message,
        Err(message) => message
    };
    println!("{}", output);
}