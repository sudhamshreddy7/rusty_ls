mod helper;
use std::env;
// use std::env;
use std::fs;
use std::path::Path;
use helper::is_substring_in_file;

#[derive(PartialEq)]
enum OPERATIONS{
    ls,
    deep_ls,
    search,
    deep_search,
    help,
    error
}
fn print_tree(path: &Path, depth: usize,flag_deep: i32,search : bool,key:&str) {
    // Print indentation based on depth
    for _ in 0..depth {
        print!("|   ");
    }

    // Print the current file/folder name
    if let Some(name) = path.file_name() {
        if search  {
            match is_substring_in_file(path.to_str().unwrap(), key) {
                Ok(true) => println!("|-- {}", name.to_string_lossy()),
                Ok(false) => todo!(),
                Err(e) => todo!(),
            }
        }
        else if !search {
            println!("|-- {}", name.to_string_lossy());
        }
    }

    // If the path is a directory, recursively print its contents
    if path.is_dir()  {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if flag_deep>0 {
                            print_tree(&entry.path(), depth + 1, flag_deep-1,search,key);
                         }
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory '{}': {}", path.display(), e),
        }
    }
}
/*
    Types of operations:
    1. regular ls
    2. deep ls --d
    3. search for a word --search "string"
    4. search for a word --search "string" --d
    5. doc --h

*/
fn main() {
    let mut type_operation = OPERATIONS::error;
    let mut key = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        if args[1] == "--d" {
            type_operation = OPERATIONS::deep_ls;
        }else if args[1] == "--h" {
            type_operation = OPERATIONS::help;
        }else if args[1] == "--search" {
            if args.len() == 2{
                panic!("Invalid flag!!\n please type rusty_ls --help for detailed commands usage");
            }
            key = args[2].clone();
            if args.len() == 3{
                type_operation = OPERATIONS::search;
            }else{
                if args[3] == "--d"{
                    type_operation = OPERATIONS::deep_search;
                }else{
                    panic!("Invalid flag!!\n please type rusty_ls --help for detailed commands usage");
                }
            }
        }
    } else{
        type_operation = OPERATIONS::ls;
    }
    let binding = env::current_dir().unwrap();
    let dir = binding.to_str().unwrap(); // Replace with the desired directory path
    let path = Path::new(dir);
    let key = key.as_str();
    if type_operation==OPERATIONS::ls {
        println!("{}",  env::current_dir().unwrap().to_str().unwrap());
        println!("List of files and directories in current directory:");
        print_tree(path, 0,1,false,key);
    }else if type_operation==OPERATIONS::deep_ls{
        println!("{}", path.display());
        println!("List of files and directories in current and below directory:");
        print_tree(path, 0,i32::max_value(),false,key);
    }else if type_operation==OPERATIONS::search{
        println!("List of files and directories which contains word {key} in current directory are:");
        print_tree(path, 0,1,true,key);
    }else if type_operation==OPERATIONS::deep_search{
        println!("List of files and directories which contains word {key} in current and below directory are:");
        print_tree(path, 0,i32::max_value(),true,key);
    }else if type_operation==OPERATIONS::help{
        help();
    }else{
        println!("please enter valid command");
        help();
    }
    // helper::get_hash_of_string("e");
    // println!("{}", path.display());
    // print_tree(path, 0);
}


fn help() {
    println!("usage: rusty_ls [OPERATION]");
}
