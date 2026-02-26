use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

use compression_tool::frequency::analyze_file;

#[test]
fn test_frequency_analysis() {
    // Create a temporary file with test content
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "aaabbc").expect("Failed to write to temp file");

    // Analyze the file
    let freq_table = analyze_file(temp_file.path().to_str().unwrap()).expect("Failed to analyze file");

    // Expected frequency table
    let mut expected = HashMap::new();
    expected.insert('a', 3);
    expected.insert('b', 2);
    expected.insert('c', 1);

    // Assert the frequency table matches the expected values
    assert_eq!(freq_table, expected);
}

#[test]
fn test_empty_file() {
    // Create an empty temporary file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");

    // Analyze the file
    let freq_table = analyze_file(temp_file.path().to_str().unwrap()).expect("Failed to analyze file");

    // Assert the frequency table is empty
    assert!(freq_table.is_empty());
}

#[test]
fn test_file_with_special_characters() {
    // Create a temporary file with special characters
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "!@#$$%^&*").expect("Failed to write to temp file");

    // Analyze the file
    let freq_table = analyze_file(temp_file.path().to_str().unwrap()).expect("Failed to analyze file");

    // Expected frequency table
    let mut expected = HashMap::new();
    expected.insert('!', 1);
    expected.insert('@', 1);
    expected.insert('#', 1);
    expected.insert('$', 2);
    expected.insert('%', 1);
    expected.insert('^', 1);
    expected.insert('&', 1);
    expected.insert('*', 1);

    // Assert the frequency table matches the expected values
    assert_eq!(freq_table, expected);
}