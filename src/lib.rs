use std::collections::HashMap;

pub fn tokenize_text(text: &str) -> Vec<u32> {
    text.as_bytes().iter().map(|&b| b as u32).collect()
}

pub fn build_vocab(tokens: &mut Vec<u32>) -> HashMap<(u32, u32), u32> {
    let mut vocab: HashMap<(u32, u32), u32> = HashMap::new();
    let mut next_id: u32 = 256;
    let mut pair_counts: HashMap<(u32, u32), usize> = HashMap::new();
    let mut new_tokens = Vec::new();

    loop {
        pair_counts.clear();
        for i in 0..tokens.len() - 1 {
            let pair = (tokens[i], tokens[i + 1]);
            *pair_counts.entry(pair).or_insert(0) += 1;
        }

        let best_pair = match pair_counts.iter().max_by_key(|&(_, count)| count) {
            Some((&pair, &count)) if count > 1 => pair,
            _ => break,
        };

        let new_id = *vocab.entry(best_pair).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        new_tokens.clear();
        let mut i = 0;
        while i < tokens.len() {
            if i + 1 < tokens.len() && (tokens[i], tokens[i + 1]) == best_pair {
                new_tokens.push(new_id);
                i += 2;
            } else {
                new_tokens.push(tokens[i]);
                i += 1;
            }
        }
        std::mem::swap(tokens, &mut new_tokens);
        // println!("  Tokens length reduced to: {}", tokens.len());
    }

    vocab
}

pub fn decode_to_base_tokens(tokens: &[u32], vocab: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    let mut reverse_vocab: HashMap<u32, (u32, u32)> = HashMap::new();
    for ((left, right), new_id) in vocab {
        reverse_vocab.insert(*new_id, (*left, *right));
    }

    let mut current_tokens: Vec<u32> = tokens.to_vec();
    let mut next_tokens = Vec::new();

    loop {
        let mut changed = false;
        next_tokens.clear();

        for &token in &current_tokens {
            if token >= 256 {
                if let Some(&(left, right)) = reverse_vocab.get(&token) {
                    next_tokens.push(left);
                    next_tokens.push(right);
                    changed = true;
                } else {
                    next_tokens.push(token);
                }
            } else {
                next_tokens.push(token);
            }
        }

        if !changed {
            break;
        }

        std::mem::swap(&mut current_tokens, &mut next_tokens);
    }

    current_tokens
}

pub fn print_tokens(tokens: &[u32]) {
    for &token in tokens {
        if token < 256 {
            print!("{}", token as u8 as char);
        } else {
            print!("[{}]", token);
        }
    }
}
