use std::env;
use std::process;

mod frequency;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Perform frequency analysis
    match frequency::analyze_file(filename) {
        Ok(freq_table) => {
            println!("Character frequencies:");
            for (ch, count) in freq_table {
                println!("{}: {}", ch, count);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}