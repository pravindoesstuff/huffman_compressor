mod heap;
mod huffman;

use crate::huffman::{build_tree, generate_weights, new_characters, Node};
use bitstream_io::{BigEndian, BitWrite, BitWriter};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut str = String::from("happy hip hop");
    str.push('\0');
    let count = generate_weights(str.clone());
    let characters = new_characters(&count);
    let tree = build_tree(characters);
    let mut ch_map = HashMap::new();
    tree_view(tree, Vec::new(), &mut ch_map);
    let bits = encode_file(&ch_map, str);
    write_tree(ch_map.clone(), "filename");
    write_bits(&bits, "file_bits");
}

fn write_tree(character_mapping: HashMap<char, Vec<u8>>, file_name: &str) {
    let mut file = File::create(file_name).unwrap();
    for (ch, bits) in character_mapping {
        let mut concat_bits = String::with_capacity(bits.len());
        for ch in bits {
            concat_bits.push((ch + '0' as u8) as char);
        }
        let data = format!("{}\n{}\n", ch, concat_bits);
        file.write_all(data.as_bytes()).unwrap();
    }
}

fn write_bits(bits: &[u8], filename: &str) {
    let mut file = File::create(filename).unwrap();
    let mut b_writer = BitWriter::endian(&mut file, BigEndian);
    for bit in bits {
        b_writer.write_bit(*bit != 0).unwrap();
    }
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
