
use std::{env, ffi::OsStr, fs::{self, DirEntry}, io::Error};

fn main() {
    let path = get_working_dir().expect("Failed to get working directory.");

    // Print the first path and then display our directories
    println!("{path}");
    display_dir(&path, 0).expect("Failed to display directory.");
}

// This is where the magic happens
fn display_dir(path: &String, level: usize) -> Result<(), Error> {

    // Read all the files from the directory, we need them to be
    // a Vector because then we can size them up, so we map and
    // collect
    let paths: Vec<(usize, Option<DirEntry>)> = fs::read_dir(path)?
        .map(|x| x.ok())
        .enumerate()
        .collect();
    let size = paths.len();

    // Based on the level, we need to generate some spacing
    let spaces = "    ".repeat(level);

    // Loop all of our paths, little bit of tuple destruction here
    for (index, entry) in paths {
        // Error handle, get the dir if it exists
        if let Some(dir) = entry {
            // Get the PathBuf object from the path
            let path = dir.path();
            // Make sure we have a file name
            if let Some(file_name) = path.file_name() {
                // Convert that file name to a string
                let file_name = os_str_as_string(file_name); 
                // If it's a directory, we want to recursively list the directories
                // and increase the level by 1
                if path.is_dir() {
                    display_dir(&path.display().to_string(), level + 1)?;
                }

                // Print the name, based on if it's at the end of the list
                // or not
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

// Little util we use to get the working directory of the user
fn get_working_dir() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.display().to_string()),
        Err(_) => None
    }
}

// Little util we can use to convert our os_str to a heap string
fn os_str_as_string(str: &OsStr) -> String {
    String::from(str.to_str().expect("Failed to convert OSStr to String"))
}