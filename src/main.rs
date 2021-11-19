mod heap;
mod huffman;

use crate::huffman::{build_tree, generate_weights, new_characters};

fn main() {
    let count = generate_weights(String::from("happy hip hop"));
    let characters = new_characters(&count);
    let tree = build_tree(characters);
    dbg!(tree);
}
