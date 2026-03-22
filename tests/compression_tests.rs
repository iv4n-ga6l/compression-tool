use std::collections::HashMap;
use std::io::Cursor;
use tempfile::NamedTempFile;
use std::fs::File;
use std::io::Write;

use compression_tool::compression::compress_and_write;
use compression_tool::tree::{build_tree, Node};
use compression_tool::header::write_header;
use compression_tool::frequency::analyze_file;

#[test]
fn test_compress_and_write() {
    // Create a temporary input file with test content
    let mut temp_input = NamedTempFile::new().expect("Failed to create temp input file");
    writeln!(temp_input, "aaabbc").expect("Failed to write to temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone()).expect("Failed to build Huffman tree");

    // Create a temporary output file
    let mut temp_output = NamedTempFile::new().expect("Failed to create temp output file");

    // Write header
    write_header(&mut temp_output, &freq_table).expect("Failed to write header");

    // Compress and write data
    compress_and_write(temp_input.path().to_str().unwrap(), &mut temp_output, &root).expect("Failed to compress and write data");

    // Read the output file
    let mut output_file = File::open(temp_output.path()).expect("Failed to open temp output file");
    let mut output_content = Vec::new();
    output_file.read_to_end(&mut output_content).expect("Failed to read output file");

    // Verify the header and compressed data
    let output_string = String::from_utf8_lossy(&output_content);
    assert!(output_string.contains("a:3"));
    assert!(output_string.contains("b:2"));
    assert!(output_string.contains("c:1"));
    assert!(output_string.contains("HEADER_END"));

    // Verify compressed data exists
    let compressed_data_start = output_string.find("HEADER_END").unwrap() + "HEADER_END\n".len();
    assert!(compressed_data_start < output_string.len());
}

#[test]
fn test_compress_empty_file() {
    // Create an empty temporary input file
    let temp_input = NamedTempFile::new().expect("Failed to create temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone());

    assert!(root.is_none());

    // Create a temporary output file
    let mut temp_output = NamedTempFile::new().expect("Failed to create temp output file");

    // Write header
    write_header(&mut temp_output, &freq_table).expect("Failed to write header");

    // Compress and write data
    let result = compress_and_write(temp_input.path().to_str().unwrap(), &mut temp_output, &root.unwrap_or_else(|| panic!("Tree should not be None")));
    assert!(result.is_err());
}

#[test]
fn test_compress_special_characters() {
    // Create a temporary input file with special characters
    let mut temp_input = NamedTempFile::new().expect("Failed to create temp input file");
    writeln!(temp_input, "!@#$$%^&*").expect("Failed to write to temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone()).expect("Failed to build Huffman tree");

    // Create a temporary output file
    let mut temp_output = NamedTempFile::new().expect("Failed to create temp output file");

    // Write header
    write_header(&mut temp_output, &freq_table).expect("Failed to write header");

    // Compress and write data
    compress_and_write(temp_input.path().to_str().unwrap(), &mut temp_output, &root).expect("Failed to compress and write data");

    // Read the output file
    let mut output_file = File::open(temp_output.path()).expect("Failed to open temp output file");
    let mut output_content = Vec::new();
    output_file.read_to_end(&mut output_content).expect("Failed to read output file");

    // Verify the header and compressed data
    let output_string = String::from_utf8_lossy(&output_content);
    assert!(output_string.contains("!:5"));
    assert!(output_string.contains("@:3"));
    assert!(output_string.contains("HEADER_END"));

    // Verify compressed data exists
    let compressed_data_start = output_string.find("HEADER_END").unwrap() + "HEADER_END\n".len();
    assert!(compressed_data_start < output_string.len());
}
