mod helper;
use std::env;
use std::fs;
use std::path::Path;
use helper::is_substring_in_file;

#[derive(PartialEq)]
enum OPERATIONS {
    Ls,
    DeepLs,
    Search,
    DeepSearch,
    Help,
    Error,
}

fn print_tree(path: &Path, depth: usize, flag_deep: i32, search: bool, key: &str) -> bool{
    // Print indentation based on depth
    for _ in 0..depth {
        print!("|   ");
    }
    let mut consider = !search;
    // Print the current file/folder name
    if let Some(name) = path.file_name() {
        if search && !path.is_dir() {
            match is_substring_in_file(path.to_str().unwrap(), key) {
                Ok(true) => {
                    println!("|-- {}", name.to_string_lossy());
                    consider = true;
                },  // Only print file if the key is found
                Ok(false) => {},
                Err(_e) => {},  // Handle any errors
            }
        } else if !search {
            println!("|-- {}", name.to_string_lossy());
        }
    }

    // If the path is a directory, recursively print its contents
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if flag_deep > 0 {
                           consider= consider || print_tree(&entry.path(), depth + 1, flag_deep - 1, search, key);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory '{}': {}", path.display(), e),
        }
    }
    consider
}

fn main() {
    let mut type_operation = OPERATIONS::Error;
    let mut key = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        if args[1] == "--d" {
            type_operation = OPERATIONS::DeepLs;
        } else if args[1] == "--h" {
            type_operation = OPERATIONS::Help;
        } else if args[1] == "--search" {
            if args.len() == 2 {
                panic!("Invalid flag!!\n please type rusty_ls --Help for detailed commands usage");
            }
            key = args[2].clone();
            if args.len() == 3 {
                type_operation = OPERATIONS::Search;
            } else {
                if args[3] == "--d" {
                    type_operation = OPERATIONS::DeepSearch;
                } else {
                    panic!("Invalid flag!!\n please type rusty_ls --Help for detailed commands usage");
                }
            }
        }
    } else {
        type_operation = OPERATIONS::Ls;
    }

    let binding = env::current_dir().unwrap();
    let dir = binding.to_str().unwrap();
    let path = Path::new(dir);
    let key = key.as_str();

    if type_operation == OPERATIONS::Ls {
        println!("{}", env::current_dir().unwrap().to_str().unwrap());
        println!("List of files and directories in the current directory:");
        print_tree(path, 0, 1, false, key);
    } else if type_operation == OPERATIONS::DeepLs {
        println!("{}", path.display());
        println!("List of files and directories in the current and below directory:");
        print_tree(path, 0, i32::max_value(), false, key);
    } else if type_operation == OPERATIONS::Search {
        println!("List of files and directories that contain the word `{}` in the current directory:", key);
        print_tree(path, 0, 1, true, key);
    } else if type_operation == OPERATIONS::DeepSearch {
        println!("List of files and directories that contain the word `{}` in the current and below directory:", key);
        print_tree(path, 0, i32::max_value(), true, key);
    } else if type_operation == OPERATIONS::Help {
        help();
    } else {
        println!("Please enter a valid command.");
        help();
    }
}

fn help() {
    println!("usage: rusty_ls [OPERATION]");
}
