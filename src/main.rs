use std::env;
use std::process;
use std::fs::File;
use std::io::Write;

mod frequency;
mod tree;
mod header;
mod compression;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_filename> <output_filename>", args[0]);
        process::exit(1);
    }

    let input_filename = &args[1];
    let output_filename = &args[2];

    // Perform frequency analysis
    let freq_table = match frequency::analyze_file(input_filename) {
        Ok(freq_table) => freq_table,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Build the binary tree
    let root = match tree::build_tree(freq_table.clone()) {
        Some(root) => root,
        None => {
            eprintln!("Failed to construct binary tree: Frequency table is empty.");
            process::exit(1);
        }
    };

    // Write header and compressed data to the output file
    let mut output_file = match File::create(output_filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = header::write_header(&mut output_file, &freq_table) {
        eprintln!("Error writing header to output file: {}", e);
        process::exit(1);
    }

    // Write compressed data
    if let Err(e) = compression::compress_and_write(input_filename, &mut output_file, &root) {
        eprintln!("Error writing compressed data to output file: {}", e);
        process::exit(1);
    }

    println!("Compression completed successfully. Output written to {}", output_filename);
}