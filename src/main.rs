use bpe::{build_vocab, decode_to_base_tokens, print_tokens, tokenize_text};

fn main() {
    let text = "The original BPE algorithm operates by iteratively replacing the most common contiguous sequences of characters in a target text with unused 'placeholder' bytes.";

    let mut tokens = tokenize_text(text);
    let vocab = build_vocab(&mut tokens);

    println!("\nVocabulary Merges:");
    for ((l, r), id) in &vocab {
        println!("  [{}, {}] -> {}", l, r, id);
    }
    // print_tokens(&tokens);
    print_tokens(&decode_to_base_tokens(&tokens, &vocab));
}
