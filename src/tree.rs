use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

/// A node in the binary tree.
#[derive(Debug, Eq)]
pub struct Node {
    pub character: Option<char>, // None for internal nodes, Some(char) for leaf nodes
    pub frequency: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    /// Creates a new leaf node.
    pub fn new_leaf(character: char, frequency: usize) -> Self {
        Node {
            character: Some(character),
            frequency,
            left: None,
            right: None,
        }
    }

    /// Creates a new internal node.
    pub fn new_internal(frequency: usize, left: Node, right: Node) -> Self {
        Node {
            character: None,
            frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make BinaryHeap a min-heap
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

/// Builds a binary tree (Huffman Tree) from a frequency table.
///
/// # Arguments
/// * `frequency_table` - A `HashMap` where the keys are characters and the values are their respective frequencies.
///
/// # Returns
/// The root node of the constructed binary tree.
pub fn build_tree(frequency_table: HashMap<char, usize>) -> Option<Node> {
    let mut heap = BinaryHeap::new();

    // Create a leaf node for each character and add it to the heap
    for (character, frequency) in frequency_table {
        heap.push(Node::new_leaf(character, frequency));
    }

    // Merge nodes until only one tree remains
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        // Create a new internal node with the combined frequency
        let merged_node = Node::new_internal(left.frequency + right.frequency, left, right);
        heap.push(merged_node);
    }

    // The remaining node is the root of the tree
    heap.pop()
}
