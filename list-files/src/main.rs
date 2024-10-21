
use std::{env, fs, io::Error};

fn main() {
    let path = get_working_dir().expect("Failed to get working directory.");
    println!("{path}");
    display_dir(&path);
}

fn get_working_dir() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.display().to_string()),
        Err(_) => None
    }
}

fn display_dir(_: &String) -> Result<(), Error> {
    let paths = fs::read_dir(".")?;
    for path in paths.into_iter() {
        println!("├── {}", path?.path().display())
    }
    Ok(())
}