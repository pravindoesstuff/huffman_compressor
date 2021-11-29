mod heap;
mod huffman;

use crate::huffman::{build_tree, generate_weights, new_characters, Node};
use std::collections::HashMap;

fn main() {
    let mut str = String::from("happy hip hop");
    str.push('\0');
    let count = generate_weights(str.clone());
    let characters = new_characters(&count);
    let tree = build_tree(characters);
    let mut ch_map = HashMap::new();
    tree_view(Box::from(tree), Vec::new(), &mut ch_map);
    let bits = encode_file(ch_map, str);
    for bit in bits {
        if bit {
            print!("0")
        } else {
            print!("1")
        }
    }
    println!();
}

fn encode_file(character_map: HashMap<char, Vec<bool>>, characters: String) -> Vec<bool> {
    let mut bitstream = Vec::new();
    for c in characters.chars() {
        bitstream.append(character_map[&c].clone().as_mut());
    }
    bitstream
}

fn tree_view(
    node: Box<Node>,
    node_list: Vec<bool>,
    character_mapping: &mut HashMap<char, Vec<bool>>,
) {
    if let Some(ch) = node.character {
        println!("{}: {:?}", ch, node_list);
        character_mapping.insert(ch, node_list);
        return;
    }
    if let Some(node) = node.left {
        let mut node_list = node_list.clone();
        node_list.push(false);
        tree_view(node, node_list, character_mapping);
    }
    if let Some(node) = node.right {
        let mut node_list = node_list.clone();
        node_list.push(true);
        tree_view(node, node_list, character_mapping);
    }
}
