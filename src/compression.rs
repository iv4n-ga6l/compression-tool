use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use crate::tree::generate_prefix_codes;
use crate::tree::Node;

/// Compresses the input file using the prefix code table and writes the compressed data to the output file.
///
/// # Arguments
/// * `input_filename` - The path to the input file to compress.
/// * `output` - A mutable reference to the output file.
/// * `root` - The root node of the Huffman tree.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
pub fn compress_and_write(input_filename: &str, output: &mut impl Write, root: &Node) -> io::Result<()> {
    let prefix_codes = generate_prefix_codes(root);

    let mut input_file = File::open(input_filename)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?; // Read the entire file into a buffer

    let mut bit_string = String::new();

    // Convert each character in the file to its corresponding prefix code
    for &byte in &buffer {
        if let Some(code) = prefix_codes.get(&(byte as char)) {
            bit_string.push_str(code);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Character not found in prefix codes"));
        }
    }

    // Pack the bit string into bytes
    let mut packed_bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for bit in bit_string.chars() {
        current_byte <<= 1;
        if bit == '1' {
            current_byte |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            packed_bytes.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    // Handle remaining bits (if any)
    if bit_count > 0 {
        current_byte <<= 8 - bit_count; // Pad remaining bits with zeros
        packed_bytes.push(current_byte);
    }

    // Write the compressed data to the output file
    output.write_all(&packed_bytes)?;

    Ok(())
}