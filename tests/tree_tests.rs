use std::collections::HashMap;

use compression_tool::tree::{build_tree, Node};

#[test]
fn test_build_tree_single_character() {
    let mut frequency_table = HashMap::new();
    frequency_table.insert('a', 5);

    let tree = build_tree(frequency_table).expect("Tree should not be None");

    assert_eq!(tree.character, Some('a'));
    assert_eq!(tree.frequency, 5);
    assert!(tree.left.is_none());
    assert!(tree.right.is_none());
}

#[test]
fn test_build_tree_multiple_characters() {
    let mut frequency_table = HashMap::new();
    frequency_table.insert('a', 5);
    frequency_table.insert('b', 9);
    frequency_table.insert('c', 12);
    frequency_table.insert('d', 13);
    frequency_table.insert('e', 16);
    frequency_table.insert('f', 45);

    let tree = build_tree(frequency_table).expect("Tree should not be None");

    // Verify the root node frequency
    assert_eq!(tree.frequency, 100); // Sum of all frequencies

    // Verify the tree structure (partial checks)
    assert!(tree.left.is_some());
    assert!(tree.right.is_some());
}

#[test]
fn test_build_tree_empty_frequency_table() {
    let frequency_table: HashMap<char, usize> = HashMap::new();

    let tree = build_tree(frequency_table);

    assert!(tree.is_none());
}
