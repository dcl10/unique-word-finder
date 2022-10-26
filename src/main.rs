use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn word_to_bits(word: &str) -> u32 {
    let letters_to_binary: HashMap<char, u32> = HashMap::from([
        ('a', 0b0000_0000_0000_0000_0000_0000_0000_0001),
        ('b', 0b0000_0000_0000_0000_0000_0000_0000_0010),
        ('c', 0b0000_0000_0000_0000_0000_0000_0000_0100),
        ('d', 0b0000_0000_0000_0000_0000_0000_0000_1000),
        ('e', 0b0000_0000_0000_0000_0000_0000_0001_0000),
        ('f', 0b0000_0000_0000_0000_0000_0000_0010_0000),
        ('g', 0b0000_0000_0000_0000_0000_0000_0100_0000),
        ('h', 0b0000_0000_0000_0000_0000_0000_1000_0000),
        ('i', 0b0000_0000_0000_0000_0000_0001_0000_0000),
        ('j', 0b0000_0000_0000_0000_0000_0010_0000_0000),
        ('k', 0b0000_0000_0000_0000_0000_0100_0000_0000),
        ('l', 0b0000_0000_0000_0000_0000_1000_0000_0000),
        ('m', 0b0000_0000_0000_0000_0001_0000_0000_0000),
        ('n', 0b0000_0000_0000_0000_0010_0000_0000_0000),
        ('o', 0b0000_0000_0000_0000_0100_0000_0000_0000),
        ('p', 0b0000_0000_0000_0000_1000_0000_0000_0000),
        ('q', 0b0000_0000_0000_0001_0000_0000_0000_0000),
        ('r', 0b0000_0000_0000_0010_0000_0000_0000_0000),
        ('s', 0b0000_0000_0000_0100_0000_0000_0000_0000),
        ('t', 0b0000_0000_0000_1000_0000_0000_0000_0000),
        ('u', 0b0000_0000_0001_0000_0000_0000_0000_0000),
        ('v', 0b0000_0000_0010_0000_0000_0000_0000_0000),
        ('w', 0b0000_0000_0100_0000_0000_0000_0000_0000),
        ('x', 0b0000_0000_1000_0000_0000_0000_0000_0000),
        ('y', 0b0000_0001_0000_0000_0000_0000_0000_0000),
        ('z', 0b0000_0010_0000_0000_0000_0000_0000_0000),
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
