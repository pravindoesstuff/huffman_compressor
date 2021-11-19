use crate::heap::Heap;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub(crate) struct Node {
    character: Option<char>,
    weight: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

// Constructs HuffmanTree with Nodes, using an array that defines character frequencies
pub(crate) fn new_characters(count: &[usize]) -> Heap<Node> {
    let mut priority_queue = Heap::new();
    for i in 0..count.len() {
        if count[i] == 0 {
            continue;
        }
        let node = Node {
            character: Some(i as u8 as char),
            weight: count[i],
            left: None,
            right: None,
        };
        priority_queue.insert(node);
    }
    priority_queue
}

pub(crate) fn generate_weights(from: String) -> Vec<usize> {
    let mut char_weight = vec![0; 256];
    for c in from.char_indices() {
        char_weight[c.1 as usize] += 1;
    }
    char_weight
}

/// Consumes a heap of Nodes, then returns a tree starting with the head node
pub(crate) fn build_tree(mut characters: Heap<Node>) -> Node {
    while characters.len() >= 2 {
        let node_1 = characters.pop().unwrap();
        let node_2 = characters.pop().unwrap();
        let new_node = Node {
            character: None,
            weight: node_1.weight + node_2.weight,
            left: Some(Box::new(node_1)),
            right: Some(Box::new(node_2)),
        };
        characters.insert(new_node);
    }
    return characters.pop().unwrap();
}
