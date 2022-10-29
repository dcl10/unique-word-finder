use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn word_to_bits(word: &str, map: &HashMap<char, u32>) -> u32 {
    let mut output = 0;
    for letter in word.chars() {
        output |= map.get(&letter).unwrap()
    }
    output
}

fn main() {
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

    let mut word_set: HashSet<u32> = HashSet::new();

    let path = Path::new("sgb-words.txt");
    let file = File::open(path).expect("uh-oh");
    let reader = BufReader::new(file);

    let now = Instant::now();
    let mut solution: Vec<String> = Vec::new();

    for line in reader.lines() {
        let word = line.unwrap();
        let word_as_bits = word_to_bits(&word, &letters_to_binary);
        word_set.insert(word_as_bits);
    }

    let elapsed = now.elapsed().as_millis();
}
