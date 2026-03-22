use std::collections::HashMap;
use std::io::{self, Write};

/// Writes the header section to the output file.
///
/// The header contains the character frequency table serialized as key-value pairs.
///
/// # Arguments
/// * `output` - A mutable reference to the output file.
/// * `freq_table` - The character frequency table.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
pub fn write_header(output: &mut impl Write, freq_table: &HashMap<char, usize>) -> io::Result<()> {
    for (character, frequency) in freq_table {
        writeln!(output, "{}:{}", character, frequency)?;
    }

    // Write a delimiter to separate the header from the compressed data
    writeln!(output, "HEADER_END")
}
