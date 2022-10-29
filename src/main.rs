use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn word_to_bits(word: &str) -> u32 {
    let letters_to_binary: HashMap<char, u32> = HashMap::from([
        ('a', 1),
        ('b', 1 << 1),
        ('c', 1 << 2),
        ('d', 1 << 3),
        ('e', 1 << 4),
        ('f', 1 << 5),
        ('g', 1 << 6),
        ('h', 1 << 7),
        ('i', 1 << 8),
        ('j', 1 << 9),
        ('k', 1 << 10),
        ('l', 1 << 11),
        ('m', 1 << 12),
        ('n', 1 << 13),
        ('o', 1 << 14),
        ('p', 1 << 15),
        ('q', 1 << 16),
        ('r', 1 << 17),
        ('s', 1 << 18),
        ('t', 1 << 19),
        ('u', 1 << 20),
        ('v', 1 << 21),
        ('w', 1 << 22),
        ('x', 1 << 23),
        ('y', 1 << 24),
        ('z', 1 << 25),
    ]);
    let mut output = 0;
    for letter in word.chars() {
        output |= letters_to_binary.get(&letter).unwrap()
    }
    output
}

fn main() {
    let path = Path::new("sgb-words.txt");
    let file = File::open(path).expect("uh-oh");
    let reader = BufReader::new(file);

    let now = Instant::now();
    let mut count = 0;
    let mut solution: Vec<String> = Vec::new();
    for line in reader.lines() {
        count += 1;
        let word = line.unwrap();
        let word_as_bits = word_to_bits(word.as_str());
        if solution.len() == 0 {
            solution.push(word)
        } else if solution.len() < 5 {
            let mut new_word_bits = 0;
            for x in solution.clone().into_iter() {
                let x_as_bits = word_to_bits(x.as_str());
                new_word_bits = word_as_bits & x_as_bits;
                if new_word_bits > 0 {
                    break;
                }
            }
            if new_word_bits == 0 {
                solution.push(word)
            }
        } else {
            break;
        }
    }
    let elapsed = now.elapsed().as_millis();
    println!("Checked {count} words in {elapsed}ms!");
    println!("{}", solution.join(", "))
}
