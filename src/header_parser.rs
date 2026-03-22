use std::collections::HashMap;
use std::io::{self, BufRead};

/// Parses the header section of an encoded file and reconstructs the frequency table.
///
/// The header is expected to be in the format:
/// <character>:<frequency>\n
/// The header ends with the line "HEADER_END".
///
/// # Arguments
/// * `input` - A buffered reader for the input file.
///
/// # Returns
/// A `Result<HashMap<char, usize>, io::Error>` containing the reconstructed frequency table.
/// Returns an error if the header is malformed or if I/O operations fail.
pub fn parse_header(input: &mut impl BufRead) -> io::Result<HashMap<char, usize>> {
    let mut frequency_table = HashMap::new();

    for line in input.lines() {
        let line = line?; // Read a line, handle potential I/O errors

        if line == "HEADER_END" {
            break; // End of header section
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Malformed header line: {}", line),
            ));
        }

        let character = parts[0].chars().next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Empty character in header")
        })?;

        let frequency: usize = parts[1].parse().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid frequency value in header line: {}", line),
            )
        })?;

        frequency_table.insert(character, frequency);
    }

    Ok(frequency_table)
}
