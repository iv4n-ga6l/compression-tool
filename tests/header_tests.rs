use std::collections::HashMap;
use std::io::Cursor;

use compression_tool::header::write_header;

#[test]
fn test_write_header() {
    let mut freq_table = HashMap::new();
    freq_table.insert('a', 3);
    freq_table.insert('b', 2);
    freq_table.insert('c', 1);

    let mut output = Cursor::new(Vec::new());

    write_header(&mut output, &freq_table).expect("Failed to write header");

    let result = String::from_utf8(output.into_inner()).expect("Failed to convert output to string");

    let expected = "a:3\nb:2\nc:1\nHEADER_END\n";
    assert_eq!(result, expected);
}

#[test]
fn test_write_header_empty_table() {
    let freq_table: HashMap<char, usize> = HashMap::new();

    let mut output = Cursor::new(Vec::new());

    write_header(&mut output, &freq_table).expect("Failed to write header");

    let result = String::from_utf8(output.into_inner()).expect("Failed to convert output to string");

    let expected = "HEADER_END\n";
    assert_eq!(result, expected);
}

#[test]
fn test_write_header_special_characters() {
    let mut freq_table = HashMap::new();
    freq_table.insert('!', 5);
    freq_table.insert('@', 3);

    let mut output = Cursor::new(Vec::new());

    write_header(&mut output, &freq_table).expect("Failed to write header");

    let result = String::from_utf8(output.into_inner()).expect("Failed to convert output to string");

    let expected = "!:5\n@:3\nHEADER_END\n";
    assert_eq!(result, expected);
}
