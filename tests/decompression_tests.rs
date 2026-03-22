use std::fs::File;
use std::io::{Read, Write};
use tempfile::NamedTempFile;
use compression_tool::decompression::decompress_file;
use compression_tool::compression::compress_and_write;
use compression_tool::header::write_header;
use compression_tool::frequency::analyze_file;
use compression_tool::tree::build_tree;

#[test]
fn test_decompress_file() {
    // Create a temporary input file with test content
    let mut temp_input = NamedTempFile::new().expect("Failed to create temp input file");
    writeln!(temp_input, "aaabbc").expect("Failed to write to temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone()).expect("Failed to build Huffman tree");

    // Create a temporary compressed file
    let mut temp_compressed = NamedTempFile::new().expect("Failed to create temp compressed file");

    // Write header
    write_header(&mut temp_compressed, &freq_table).expect("Failed to write header");

    // Compress and write data
    compress_and_write(temp_input.path().to_str().unwrap(), &mut temp_compressed, &root).expect("Failed to compress and write data");

    // Create a temporary decompressed file
    let temp_decompressed = NamedTempFile::new().expect("Failed to create temp decompressed file");

    // Decompress the file
    decompress_file(temp_compressed.path().to_str().unwrap(), temp_decompressed.path().to_str().unwrap()).expect("Failed to decompress file");

    // Read the decompressed file
    let mut decompressed_content = String::new();
    let mut decompressed_file = File::open(temp_decompressed.path()).expect("Failed to open temp decompressed file");
    decompressed_file.read_to_string(&mut decompressed_content).expect("Failed to read decompressed file");

    // Verify the decompressed content matches the original input
    assert_eq!(decompressed_content, "aaabbc\n");
}

#[test]
fn test_decompress_empty_file() {
    // Create an empty temporary input file
    let temp_input = NamedTempFile::new().expect("Failed to create temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone());

    assert!(root.is_none());

    // Create a temporary compressed file
    let mut temp_compressed = NamedTempFile::new().expect("Failed to create temp compressed file");

    // Write header
    write_header(&mut temp_compressed, &freq_table).expect("Failed to write header");

    // Create a temporary decompressed file
    let temp_decompressed = NamedTempFile::new().expect("Failed to create temp decompressed file");

    // Decompress the file
    decompress_file(temp_compressed.path().to_str().unwrap(), temp_decompressed.path().to_str().unwrap()).expect("Failed to decompress file");

    // Read the decompressed file
    let mut decompressed_content = String::new();
    let mut decompressed_file = File::open(temp_decompressed.path()).expect("Failed to open temp decompressed file");
    decompressed_file.read_to_string(&mut decompressed_content).expect("Failed to read decompressed file");

    // Verify the decompressed content is empty
    assert!(decompressed_content.is_empty());
}

#[test]
fn test_decompress_special_characters() {
    // Create a temporary input file with special characters
    let mut temp_input = NamedTempFile::new().expect("Failed to create temp input file");
    writeln!(temp_input, "!@#$$%^&*").expect("Failed to write to temp input file");

    // Analyze the file and build the Huffman tree
    let freq_table = analyze_file(temp_input.path().to_str().unwrap()).expect("Failed to analyze file");
    let root = build_tree(freq_table.clone()).expect("Failed to build Huffman tree");

    // Create a temporary compressed file
    let mut temp_compressed = NamedTempFile::new().expect("Failed to create temp compressed file");

    // Write header
    write_header(&mut temp_compressed, &freq_table).expect("Failed to write header");

    // Compress and write data
    compress_and_write(temp_input.path().to_str().unwrap(), &mut temp_compressed, &root).expect("Failed to compress and write data");

    // Create a temporary decompressed file
    let temp_decompressed = NamedTempFile::new().expect("Failed to create temp decompressed file");

    // Decompress the file
    decompress_file(temp_compressed.path().to_str().unwrap(), temp_decompressed.path().to_str().unwrap()).expect("Failed to decompress file");

    // Read the decompressed file
    let mut decompressed_content = String::new();
    let mut decompressed_file = File::open(temp_decompressed.path()).expect("Failed to open temp decompressed file");
    decompressed_file.read_to_string(&mut decompressed_content).expect("Failed to read decompressed file");

    // Verify the decompressed content matches the original input
    assert_eq!(decompressed_content, "!@#$$%^&*\n");
}