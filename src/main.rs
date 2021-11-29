mod heap;
mod huffman;

use crate::huffman::{build_tree, generate_weights, new_characters, Node};
use bitstream_io::{huffman::compile_read_tree, BigEndian};
use std::collections::HashMap;

fn main() {
    let mut str = String::from("happy hip hop");
    str.push('\0');
    let count = generate_weights(str.clone());
    let characters = new_characters(&count);
    let tree = build_tree(characters);
    let mut ch_map = HashMap::new();
    tree_view(tree, Vec::new(), &mut ch_map);
    let bits = encode_file(&ch_map, str);
    write_tree(ch_map);
    println!("{:#?}", bits);
}

fn write_tree(character_mapping: HashMap<char, Vec<u8>>) {
    print!("{:#?}", character_mapping);
    let mut character_vec = Vec::new();
    for (ch, bits) in character_mapping {
        character_vec.push((ch, bits));
    }
    character_vec.sort_by(|a, b| a.1.len().partial_cmp(&b.1.len()).unwrap()); //Vector must be sorted prior to compilation
    let compiled_tree = compile_read_tree::<BigEndian, char>(character_vec);
}

fn encode_file(character_map: &HashMap<char, Vec<u8>>, characters: String) -> Vec<u8> {
    let mut bitstream = Vec::new();
    for c in characters.chars() {
        bitstream.append(character_map[&c].clone().as_mut());
    }
    bitstream
}

fn tree_view(node: Node, node_list: Vec<u8>, character_mapping: &mut HashMap<char, Vec<u8>>) {
    if let Some(ch) = node.character {
        character_mapping.insert(ch, node_list);
        return;
    }
    if let Some(node) = node.left {
        let mut node_list = node_list.clone();
        node_list.push(0);
        tree_view(*node, node_list, character_mapping);
    }
    if let Some(node) = node.right {
        let mut node_list = node_list;
        node_list.push(1);
        tree_view(*node, node_list, character_mapping);
    }
}
