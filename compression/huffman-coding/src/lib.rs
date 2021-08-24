use std::collections::HashMap;
use std::vec::Vec;

fn distribution(input: &str) -> Vec<(usize, char)> {
    let mut count_map = HashMap::new();

    for c in input.chars() {
        *count_map.entry(c).or_insert(0usize) += 1;
    }

    count_map
        .into_iter()
        .map(|(char, freq)| (freq, char))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct HuffmanNode {
    freq: usize,
    char: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new(freq: usize, char: Option<char>) -> Self {
        HuffmanNode {
            freq,
            char,
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, node: HuffmanNode) {
        self.left = Some(Box::new(node));
    }

    pub fn insert_right(&mut self, node: HuffmanNode) {
        self.right = Some(Box::new(node));
    }
}

fn huffman_codes(distribution: &mut [(usize, char)]) -> HashMap<char, usize> {
    distribution.sort_by(|a, b| a.0.cmp(&b.0));

    let mut it = distribution.iter();
    let mut root: Option<HuffmanNode> = None;

    while let Some(first) = it.next() {
        let node = HuffmanNode::new(first.0, Some(first.1));

        if let Some(root_node) = root {
            let mut parent = HuffmanNode::new(root_node.freq + first.0, None);

            if root_node.freq < first.0 {
                parent.insert_left(root_node);
                parent.insert_right(node);
            } else {
                parent.insert_left(node);
                parent.insert_right(root_node);
            }

            root = Some(parent);
        } else if let Some(second) = it.next() {
            let mut parent = HuffmanNode::new(first.0 + second.0, None);

            parent.insert_left(HuffmanNode::new(first.0, Some(first.1)));
            parent.insert_right(HuffmanNode::new(second.0, Some(second.1)));

            root = Some(parent);
        }
    }

    let mut codes = HashMap::new();
    add_huffman_codes(&root.unwrap(), String::from(""), &mut codes);

    codes
}

fn add_huffman_codes(root: &HuffmanNode, prefix: String, codes: &mut HashMap<char, usize>) {
    if root.left.is_none() && root.right.is_none() && root.char.is_some() {
        codes.insert(
            root.char.unwrap(),
            usize::from_str_radix(&prefix, 2).unwrap(),
        );
        return;
    }

    if let Some(left_node) = &root.left {
        let prefix = format!("{}{}", prefix, "0");
        add_huffman_codes(left_node.as_ref(), prefix, codes);
    }

    if let Some(right_node) = &root.right {
        let prefix = format!("{}{}", prefix, "1");
        add_huffman_codes(right_node.as_ref(), prefix, codes);
    }
}

pub fn encode(input: &str) -> (Vec<usize>, HashMap<usize, char>) {
    let distribution = distribution(input);
    let codes = huffman_codes(&mut distribution.to_owned());

    let encoded = input
        .chars()
        .map(|char| *codes.get(&char).unwrap())
        .collect::<Vec<_>>();

    let mut decode_map = HashMap::new();
    for (char, code) in codes.iter() {
        decode_map.insert(*code, *char);
    }

    (encoded, decode_map)
}

pub fn decode(input: &[usize], decode_map: &HashMap<usize, char>) -> String {
    input
        .iter()
        .map(|byte| decode_map.get(byte).unwrap())
        .collect::<String>()
}

#[cfg(test)]
pub mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{self, thread_rng, Rng};

    use super::{decode, encode};

    #[test]
    pub fn huffman_works() {
        let rng = &mut thread_rng();

        for _ in 0..5000 {
            let message: String = rng
                .sample_iter(&Alphanumeric)
                .take(50)
                .map(char::from)
                .collect();

            println!("Encoding message {} (size={})", message, message.len());

            let (encoded, decode_map) = encode(&message);
            println!("Encoded message size={:?}", encoded.len());

            let decoded = decode(&encoded, &decode_map);

            assert_eq!(message, decoded)
        }
    }
}
