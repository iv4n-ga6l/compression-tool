use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use crate::header_parser::parse_header;
use crate::tree::{build_tree, Node};

/// Decompresses the input file and writes the decoded text to the output file.
///
/// # Arguments
/// * `input_filename` - The path to the compressed input file.
/// * `output_filename` - The path to the output file where the decompressed text will be written.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
pub fn decompress_file(input_filename: &str, output_filename: &str) -> io::Result<()> {
    let input_file = File::open(input_filename)?;
    let mut reader = BufReader::new(input_file);

    // Parse the header to reconstruct the frequency table
    let freq_table = parse_header(&mut reader)?;

    // Build the Huffman tree from the frequency table
    let root = build_tree(freq_table).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Failed to reconstruct Huffman tree")
    })?;

    // Read the remainder of the file (compressed data)
    let mut compressed_data = Vec::new();
    reader.read_to_end(&mut compressed_data)?;

    // Decode the compressed data using the Huffman tree
    let decoded_text = decode_data(&compressed_data, &root)?;

    // Write the decoded text to the output file
    let mut output_file = File::create(output_filename)?;
    output_file.write_all(decoded_text.as_bytes())?;

    Ok(())
}

/// Decodes the compressed data using the Huffman tree.
///
/// # Arguments
/// * `compressed_data` - The compressed data as a byte vector.
/// * `root` - The root node of the Huffman tree.
///
/// # Returns
/// A `Result<String, io::Error>` containing the decoded text.
pub fn decode_data(compressed_data: &[u8], root: &Node) -> io::Result<String> {
    let mut decoded_text = String::new();
    let mut current_node = root;

    for byte in compressed_data {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;

            // Traverse the Huffman tree based on the current bit
            current_node = if bit == 0 {
                current_node.left.as_ref().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Malformed Huffman tree traversal")
                })?
            } else {
                current_node.right.as_ref().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Malformed Huffman tree traversal")
                })?
            };

            // If we reach a leaf node, append the character to the decoded text
            if let Some(character) = current_node.character {
                decoded_text.push(character);
                current_node = root; // Reset to the root for the next character
            }
        }
    }

    Ok(decoded_text)
}