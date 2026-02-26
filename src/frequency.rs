use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Analyzes the frequency of characters in a given file.
///
/// # Arguments
/// * `filename` - The path to the file to analyze.
///
/// # Returns
/// A `HashMap` where the keys are characters and the values are their respective frequencies.
/// Returns an `io::Result` to handle potential file I/O errors.
pub fn analyze_file(filename: &str) -> io::Result<HashMap<char, usize>> {
    let file = File::open(filename)?; // Open the file, return error if it fails
    let reader = BufReader::new(file);

    let mut frequency_table: HashMap<char, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors
        for ch in line.chars() {
            *frequency_table.entry(ch).or_insert(0) += 1; // Increment character count
        }
    }

    Ok(frequency_table)
}