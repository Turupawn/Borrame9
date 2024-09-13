use std::env;
use std::fs;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: myplugin <directory_name>");
        process::exit(1);
    }

    let directory_name = &args[1];

    // Attempt to create the directory
    match fs::create_dir(directory_name) {
        Ok(_) => println!("Directory '{}' created successfully.", directory_name),
        Err(e) => {
            eprintln!("Failed to create directory '{}': {}", directory_name, e);
            process::exit(1);
        }
    }
}