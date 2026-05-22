use std::collections::HashMap;

fn print_tokens(tokens: &[u32]) {
    for &token in tokens {
        if token < 256 {
            print!("{}", token as u8 as char);
        } else {
            print!("[{}]", token);
        }
    }
}
/// 根据最终 tokens 和 vocab 反向解码为基础 Token ID 列表 (Vec<u32>)
fn decode_to_base_tokens(tokens: &[u32], vocab: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    // 1. 构建反向映射: new_id -> (left_id, right_id)

    let mut reverse_vocab: HashMap<u32, (u32, u32)> = HashMap::new();
    for ((left, right), new_id) in vocab {
        reverse_vocab.insert(*new_id, (*left, *right));
    }

    // 2. 初始化当前 token 列表
    let mut current_tokens: Vec<u32> = tokens.to_vec();

    // 3. 迭代展开，直到没有复合 token (id >= 256)
    loop {
        let mut changed = false;
        let mut next_tokens: Vec<u32> = Vec::new();

        for &token in &current_tokens {
            if token >= 256 {
                // 如果这个 token 是在 vocab 中生成的，则展开它
                if let Some(&(left, right)) = reverse_vocab.get(&token) {
                    next_tokens.push(left);
                    next_tokens.push(right);
                    changed = true;
                } else {
                    // 理论上不应该发生，除非 vocab 不完整或 token 无效
                    next_tokens.push(token);
                }
            } else {
                // 基础字符，直接保留
                next_tokens.push(token);
            }
        }

        // 更新 tokens 列表
        current_tokens = next_tokens;

        // 如果这一轮没有任何展开操作，说明所有 token 都是基础字符了
        if !changed {
            break;
        }
    }

    // 4. 直接返回基础 ID 列表
    current_tokens
}

fn main() {
    let text = "The original BPE algorithm operates by iteratively replacing the most common contiguous sequences of characters in a target text with unused 'placeholder' bytes.";

    // 1. 初始化：将文本转换为 Token ID 列表 (ASCII 值)
    let mut tokens: Vec<u32> = text.as_bytes().iter().map(|&b| b as u32).collect();

    // 2. 词汇表：记录 (left_id, right_id) -> new_id
    let mut vocab: HashMap<(u32, u32), u32> = HashMap::new();
    let mut next_id: u32 = 256; // 从 ASCII 范围之后开始

    loop {
        // 3. 统计所有相邻 Pair 的频率
        let mut pair_counts: HashMap<(u32, u32), usize> = HashMap::new();
        for i in 0..tokens.len() - 1 {
            let pair = (tokens[i], tokens[i + 1]);
            *pair_counts.entry(pair).or_insert(0) += 1;
        }

        // 4. 找到最高频的 Pair
        let best_pair = match pair_counts.iter().max_by_key(|&(_, count)| count) {
            Some((&pair, &count)) if count > 1 => pair,
            _ => break, // 没有频率大于 1 的 pair，结束
        };

        // println!(
        //     ": Merging {:?} (freq: {})",
        //     best_pair, pair_counts[&best_pair]
        // );

        // 5. 分配新 ID
        let new_id = *vocab.entry(best_pair).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        // 6. 更新 tokens 列表：替换 best_pair 为 new_id
        let mut new_tokens = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            if i + 1 < tokens.len() && (tokens[i], tokens[i + 1]) == best_pair {
                new_tokens.push(new_id);
                i += 2; // 跳过这两个已合并的 token
            } else {
                new_tokens.push(tokens[i]);
                i += 1;
            }
        }
        tokens = new_tokens;
        println!("  Tokens length reduced to: {}", tokens.len());
    }

    // 输出最终的词汇表映射
    println!("\nVocabulary Merges:");
    for ((l, r), id) in &vocab {
        println!("  [{}, {}] -> {}", l, r, id);
    }
    //输出 最终的 token ID 列表
    // println!("\nFinal Token IDs:");
    // println!("{:?}", tokens);

    print_tokens(&tokens);
    print_tokens(&decode_to_base_tokens(&tokens, &vocab));
}
