use std::collections::HashMap;
use std::io::Cursor;

use compression_tool::header_parser::parse_header;

#[test]
fn test_parse_valid_header() {
    let header_content = "a:3\nb:2\nc:1\nHEADER_END\n";
    let mut input = Cursor::new(header_content);

    let freq_table = parse_header(&mut input).expect("Failed to parse header");

    let mut expected = HashMap::new();
    expected.insert('a', 3);
    expected.insert('b', 2);
    expected.insert('c', 1);

    assert_eq!(freq_table, expected);
}

#[test]
fn test_parse_empty_header() {
    let header_content = "HEADER_END\n";
    let mut input = Cursor::new(header_content);

    let freq_table = parse_header(&mut input).expect("Failed to parse header");

    assert!(freq_table.is_empty());
}

#[test]
fn test_parse_malformed_header_missing_delimiter() {
    let header_content = "a3\nb:2\nHEADER_END\n";
    let mut input = Cursor::new(header_content);

    let result = parse_header(&mut input);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn test_parse_malformed_header_missing_end() {
    let header_content = "a:3\nb:2\nc:1\n";
    let mut input = Cursor::new(header_content);

    let result = parse_header(&mut input);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
}

#[test]
fn test_parse_header_with_special_characters() {
    let header_content = "!:5\n@:3\nHEADER_END\n";
    let mut input = Cursor::new(header_content);

    let freq_table = parse_header(&mut input).expect("Failed to parse header");

    let mut expected = HashMap::new();
    expected.insert('!', 5);
    expected.insert('@', 3);

    assert_eq!(freq_table, expected);
}
