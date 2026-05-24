use std::collections::HashMap;
use std::time::Instant;

fn update_pair_count(pair_counts: &mut HashMap<(u32, u32), u32>, pair: (u32, u32), delta: i32) {
    if delta == 0 {
        return;
    }

    match pair_counts.get_mut(&pair) {
        Some(count) => {
            if delta < 0 {
                let drop = (-delta) as u32;
                if *count > drop {
                    *count -= drop;
                } else {
                    pair_counts.remove(&pair);
                }
            } else {
                *count += delta as u32;
            }
        }
        None if delta > 0 => {
            pair_counts.insert(pair, delta as u32);
        }
        _ => {}
    }
}
pub fn tokenize_text(text: &str) -> Vec<u32> {
    text.as_bytes().iter().map(|&b| b as u32).collect()
}

const THRESHOLD: u32 = 10;
pub fn build_vocab(tokens: &mut Vec<u32>) -> HashMap<(u32, u32), u32> {
    let mut vocab = HashMap::new();
    let mut next_id = 256;

    let mut pair_counts = HashMap::with_capacity(tokens.len());
    let mut new_tokens = Vec::with_capacity(tokens.len());

    let mut start = Instant::now();
    // 每一次迭代不重新扫描全部 pair_counts，而是更新被替换 pair 及其左右邻居的计数，并移除计数归零的 entry。

    let mut count = 0;

    for window in tokens.windows(2) {
        *pair_counts.entry((window[0], window[1])).or_insert(0) += 1;
    }

    println!("分词结束: {:?} ", start.elapsed());
    start = Instant::now();

    loop {
        let best_pair = match pair_counts.iter().max_by_key(|&(_, c)| c) {
            Some((pair, count)) if *count > 1 => *pair,
            _ => {
                println!("创建: {:?}", start.elapsed());
                break;
            }
        };
        // println!("best_pair: {:?}", best_pair);
        // println!("pair_counts: {:?}", pair_counts.len());

        let new_id = *vocab.entry(best_pair).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        if count % THRESHOLD == 0 {
            println!(
                "创建时间: {:?} tokens length:{} pair_counts length:{}",
                start.elapsed(),
                tokens.len(),
                pair_counts.len()
            );
            start = Instant::now();
        }

        let mut i = 0;
        new_tokens.clear();
        while i < tokens.len() {
            if i < tokens.len() - 1 && (tokens[i], tokens[i + 1]) == best_pair {
                // 1. 移除受影响的旧 pairs
                // 左边的 pair (如果有)
                if i > 0 {
                    update_pair_count(&mut pair_counts, (tokens[i - 1], tokens[i]), -1);
                }
                // 当前的 pair
                update_pair_count(&mut pair_counts, best_pair, -1);

                // 右边的 pair (如果有)
                if i + 2 < tokens.len() {
                    update_pair_count(&mut pair_counts, (tokens[i + 1], tokens[i + 2]), -1);
                }

                // 2. 添加新 token
                new_tokens.push(new_id);

                // 3. 添加新的 pairs
                // 左边的新 pair (如果有)
                if new_tokens.len() >= 2 {
                    let left_pair = (new_tokens[new_tokens.len() - 2], new_id);
                    update_pair_count(&mut pair_counts, left_pair, 1);
                }

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
