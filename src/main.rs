use std::env;
use std::process;

mod frequency;
mod tree;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Perform frequency analysis
    let freq_table = match frequency::analyze_file(filename) {
        Ok(freq_table) => freq_table,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Build the binary tree
    match tree::build_tree(freq_table) {
        Some(root) => {
            println!("Binary tree constructed successfully.");
            println!("Root frequency: {}", root.frequency);
        }
        None => {
            eprintln!("Failed to construct binary tree: Frequency table is empty.");
            process::exit(1);
        }
    }
}