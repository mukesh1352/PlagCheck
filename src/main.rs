mod input;
mod content_checker;

use crate::input::file_inputter;
use std::io;

fn main() {
    println!("Enter the path of the file to be compared:");
    let mut pathname = String::new();
    io::stdin()
        .read_line(&mut pathname)
        .expect("Failed to read file path.");

    println!("Enter the comparison directory:");
    let mut outputdir = String::new();
    io::stdin()
        .read_line(&mut outputdir)
        .expect("Failed to read directory path.");

    let trimmed_path = pathname.trim();
    let trimmed_output_path = outputdir.trim();

    let result = file_inputter(trimmed_path, trimmed_output_path);

    if result {
        println!("\n Success.");
    } else {
        println!("\n Failure. Please check your paths.");
    }
}
