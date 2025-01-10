// use std::env;
use std::fs;
use std::path::Path;

fn print_tree(path: &Path, depth: usize) {
    // Print indentation based on depth
    for _ in 0..depth {
        print!("|   ");
    }

    // Print the current file/folder name
    if let Some(name) = path.file_name() {
        println!("|-- {}", name.to_string_lossy());
    }

    // If the path is a directory, recursively print its contents
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        print_tree(&entry.path(), depth + 1);
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory '{}': {}", path.display(), e),
        }
    }
}

fn main() {
    let dir = "."; // Replace with the desired directory path
    let path = Path::new(dir);

    println!("{}", path.display());
    print_tree(path, 0);
}
