use std::collections::HashMap;

use compression_tool::tree::{build_tree, generate_prefix_codes};

#[test]
fn test_generate_prefix_codes_single_character() {
    let mut frequency_table = HashMap::new();
    frequency_table.insert('a', 5);

    let tree = build_tree(frequency_table).expect("Tree should not be None");
    let codes = generate_prefix_codes(&tree);

    let mut expected = HashMap::new();
    expected.insert('a', "".to_string());

    assert_eq!(codes, expected);
}

#[test]
fn test_generate_prefix_codes_multiple_characters() {
    let mut frequency_table = HashMap::new();
    frequency_table.insert('a', 5);
    frequency_table.insert('b', 9);
    frequency_table.insert('c', 12);
    frequency_table.insert('d', 13);
    frequency_table.insert('e', 16);
    frequency_table.insert('f', 45);

    let tree = build_tree(frequency_table).expect("Tree should not be None");
    let codes = generate_prefix_codes(&tree);

    // Verify prefix codes (partial checks)
    assert!(codes.contains_key('a'));
    assert!(codes.contains_key('b'));
    assert!(codes.contains_key('c'));
    assert!(codes.contains_key('d'));
    assert!(codes.contains_key('e'));
    assert!(codes.contains_key('f'));

    // Ensure prefix property
    for (key, code) in &codes {
        for (other_key, other_code) in &codes {
            if key != other_key {
                assert!(!other_code.starts_with(code));
            }
        }
    }
}

#[test]
fn test_generate_prefix_codes_empty_tree() {
    let frequency_table: HashMap<char, usize> = HashMap::new();

    let tree = build_tree(frequency_table);

    assert!(tree.is_none());
}
