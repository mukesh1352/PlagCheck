mod input;
use crate::input::file_inputter; 
use std::io;

fn main() {
    println!("Enter the path of the file to be compared..\n");
    let mut pathname = String::new();
    io::stdin().read_line(&mut pathname).expect("Failed to take the path of the file...");

    println!("Enter the comparison directory...\n");
    let mut outputdir = String::new();
    io::stdin().read_line(&mut outputdir).expect("Failed to take the path of the directory...");

    let trimmed_path = pathname.trim();
    let trimmed_output_path = outputdir.trim();

    let result: bool = file_inputter(trimmed_path, trimmed_output_path);
    if result {
        println!("Success..");
    } else {
        println!("Failure..");
    }
}
