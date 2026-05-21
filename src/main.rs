use std::collections::HashMap;

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

        println!(
            ": Merging {:?} (freq: {})",
            best_pair, pair_counts[&best_pair]
        );

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
}
