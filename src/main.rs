use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Token {
    l: u32,
    r: u32,
}
impl Token {
    fn new(l: u32, r: u32) -> Self {
        Self { l, r }
    }
}

#[allow(dead_code)]
fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("[{}, {}]=>{:?}", token.l, token.r, token);
    }
}
#[allow(dead_code)]
fn print_tokens_frequency(tokens_frequency: &HashMap<Token, usize>, n: usize) {
    if tokens_frequency.is_empty() {
        return;
    }

    // 1. 将 HashMap 转换为 Vec，以便进行排序
    let mut sorted_tokens: Vec<(&Token, &usize)> = tokens_frequency.iter().collect();

    // 2. 按频率降序排序 (b.1.cmp(a.1) 表示降序，即大的在前)
    sorted_tokens.sort_by(|a, b| b.1.cmp(a.1));

    // 3. 取前 n 个 (如果总数不足 n，则取全部)
    let top_n = &sorted_tokens[..std::cmp::min(n, sorted_tokens.len())];

    // 4. 打印结果
    for (token, frequency) in top_n {
        println!("token_frequency:[{}, {}]: {}", token.l, token.r, frequency);
    }
}

fn main() {
    let text = "The original BPE algorithm operates by iteratively replacing the most common contiguous sequences of characters in a target text with unused 'placeholder' bytes. The iteration ends when no sequences can be found, leaving the target text effectively compressed. Decompression can be performed by reversing this process, querying known placeholder terms against their corresponding denoted sequence, using a lookup table. In the original paper, this lookup table is encoded and stored alongside the compressed text.";
    const MAX_COUNT: i32 = 2;
    const ENABLE_MAX_COUNT: bool = false;
    let mut tokens_in: Vec<Token> = text
        .as_bytes()
        .windows(2)
        .map(|c| Token::new(c[0] as u32, c[1] as u32))
        .collect::<Vec<Token>>();

    let mut tokens_frequency: HashMap<Token, usize> =
        tokens_in.iter().fold(HashMap::new(), |mut acc, token| {
            *acc.entry(*token).or_insert(0) += 1;
            acc
        });
    let mut tokens_out: Vec<Token> = Vec::new();
    let mut count = 0;
    let mut new_char_index = 256;
    let mut tokens_map: HashMap<Token, u32> = HashMap::new();

    loop {
        let (token, most_frequent) = tokens_frequency.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

        print_tokens_frequency(&tokens_frequency, 1);
        if *most_frequent <= 1 {
            break;
        }

        let mut i = 0;
        tokens_out.clear();
        while i < tokens_in.len() - 1 {
            if tokens_in[i] == *token
                && tokens_in[i].l < 255
                && i + 1 <= tokens_in.len() - 1
                && tokens_in[i + 1].r <= 255
            {
                tokens_map.insert(*token, new_char_index);
                tokens_out.push(Token::new(new_char_index, tokens_in[i + 1].r));
                i += 2;
            } else {
                tokens_out.push(tokens_in[i]);
                i += 1;
            }
        }

        println!("Count: {}", count);

        if count >= MAX_COUNT && ENABLE_MAX_COUNT {
            break;
        }

        tokens_frequency.clear();
        tokens_frequency = tokens_out.iter().fold(HashMap::new(), |mut acc, token| {
            *acc.entry(*token).or_insert(0) += 1;
            acc
        });

        new_char_index += 1;
        count += 1;

        std::mem::swap(&mut tokens_in, &mut tokens_out);
    }

    for token in &tokens_map {
        println!("token_map:[{}, {}]: {}", token.0.l, token.0.r, token.1);
    }
    // print_tokens(&tokens_out);
    // print_tokens_frequency(&tokens_frequency);
}
