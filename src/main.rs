use std::fs;

use bpe::{build_vocab, decode_to_base_tokens, print_tokens, tokenize_text};

fn main() {
    // let text = "The original BPE algorithm operates by iteratively replacing the most common contiguous sequences of characters in a target text with unused 'placeholder' bytes.";

    let text = fs::read_to_string("test_500k.txt").expect("无法读取文件");

    let mut tokens = tokenize_text(&text);
    let vocab = build_vocab(&mut tokens);

    // println!("\nVocabulary Merges:");
    // for ((l, r), id) in &vocab {
    //     println!("  [{}, {}] -> {}", l, r, id);
    // }
    // print_tokens(&tokens);
    // print_tokens(&decode_to_base_tokens(&tokens, &vocab));
}
