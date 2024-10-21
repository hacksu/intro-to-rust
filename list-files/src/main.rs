
use std::{env, ffi::OsStr, fs::{self, DirEntry, ReadDir}, io::Error};

fn main() {
    let path = get_working_dir().expect("Failed to get working directory.");
    println!("{path}");
    display_dir(&path, 0).expect("asd");
}

fn get_working_dir() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.display().to_string()),
        Err(_) => None
    }
}

fn os_str_as_string(str: &OsStr) -> String {
    String::from(str.to_str().expect("Failed to convert OSStr to String"))
}

fn display_dir(path: &String, level: usize) -> Result<(), Error> {
    let paths: Vec<(usize, Option<DirEntry>)> = fs::read_dir(path)?
        .map(|x| x.ok())
        .enumerate()
        .collect();
    let size = paths.len();
    let spaces = "    ".repeat(level);
    for (index, entry) in paths {
        if let Some(dir) = entry {
            let path = dir.path();
            if let Some(file_name) = path.file_name() {
                let file_name = os_str_as_string(file_name);
                if (path.is_dir()) {
                    display_dir(&path.display().to_string(), level + 1)?;
                }
                if index + 1 == size {
                    println!("{spaces}└── {file_name}");
                } else {
                    println!("{spaces}├── {file_name}");
                }
            }
        }
    }
    Ok(())
}