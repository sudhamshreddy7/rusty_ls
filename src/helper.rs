use std::fs::File;
use std::io::{BufReader, BufRead, Result};
// Function to check if a substring is present in the file
pub(crate) fn is_substring_in_file(file_path: &str, target: &str) -> Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;

        // Check if the target substring is in the current line
        if line.contains(target) {
            return Ok(true);  // Substring found
        }
    }

    Ok(false)  // Substring not found
}

fn main() {
    let file_path = "sample.txt";  // Specify the path to your file
    let target_string = "Rust";    // Specify the substring you want to Search for

    match is_substring_in_file(file_path, target_string) {
        Ok(true) => println!("The substring '{}' is present in the file.", target_string),
        Ok(false) => println!("The substring '{}' is not present in the file.", target_string),
        Err(e) => println!("Error reading the file: {}", e),
    }
}
