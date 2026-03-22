use std::collections::HashMap;

/// Generates a prefix-code table from the Huffman tree.
///
/// # Arguments
/// * `root` - The root node of the Huffman tree.
///
/// # Returns
/// A `HashMap` where keys are characters and values are their corresponding binary codes.
pub fn generate_prefix_codes(root: &Node) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    
    /// Helper function to recursively traverse the tree and build codes.
    fn traverse(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
        if let Some(character) = node.character {
            // Leaf node: store the code for the character
            codes.insert(character, prefix);
        } else {
            // Internal node: traverse left and right children
            if let Some(ref left) = node.left {
                traverse(left, format!("{}0", prefix), codes);
            }
            if let Some(ref right) = node.right {
                traverse(right, format!("{}1", prefix), codes);
            }
        }
    }

    // Start traversal from the root
    traverse(root, String::new(), &mut codes);
    codes
}
