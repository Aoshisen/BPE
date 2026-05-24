use std::collections::HashMap;
use std::time::Instant;

pub fn tokenize_text(text: &str) -> Vec<u32> {
    text.as_bytes().iter().map(|&b| b as u32).collect()
}

pub fn build_vocab(tokens: &mut Vec<u32>) -> HashMap<(u32, u32), u32> {
    let mut vocab = HashMap::new();
    let mut next_id = 256;

    let mut pair_counts = HashMap::with_capacity(tokens.len());
    let mut new_tokens = Vec::with_capacity(tokens.len());
    let mut count = 0;

    let start = Instant::now();
    // 每一次迭代 不清除 pair_counts, 只更新被替换的pair的count, 这样可以大幅提升性能

    for window in tokens.windows(2) {
        *pair_counts.entry((window[0], window[1])).or_insert(0) += 1;
    }
    loop {
        let best_pair = match pair_counts.iter().max_by_key(|&(_, c)| c) {
            Some((pair, count)) if *count > 1 => *pair,
            _ => break,
        };

        let new_id = *vocab.entry(best_pair).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        if count >= 1000 {
            println!("创建: {:?}", start.elapsed());
            break;
        }

        let mut i = 0;
        new_tokens.clear();
        while i < tokens.len() {
            if i + 1 < tokens.len() && (tokens[i], tokens[i + 1]) == best_pair {
                // aabc
                //  aXc
                // 1. 找到new_tokens 的钱一个char,然后组合 成一个pair 如果在 pairs_count 中,就把这个pair + 1; 如果不存在 那么创建一个new_id;
                // 2. 当前pair_counts 中, 删除 (tokens[i], tokens[i+1]) 这个pair 的count -1;
                if new_tokens.len() > 0 {
                    let last_token = *new_tokens.last().unwrap();
                    *pair_counts.entry((last_token, new_id)).or_insert(0) += 1;
                }
                pair_counts
                    .entry((tokens[i], tokens[i + 1]))
                    .and_modify(|c| *c -= 1);
                new_tokens.push(new_id);
                i += 2;
            } else {
                new_tokens.push(tokens[i]);
                i += 1;
            }
        }
        count += 1;
        std::mem::swap(tokens, &mut new_tokens);
    }

    vocab
}

pub fn decode_to_base_tokens(tokens: &[u32], vocab: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    let mut reverse_vocab: HashMap<u32, (u32, u32)> = HashMap::with_capacity(vocab.len());
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
