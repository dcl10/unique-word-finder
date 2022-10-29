use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn word_to_bits(word: &str, map: &HashMap<char, u32>) -> Option<u32> {
    let mut output = 0;
    let mut char_set = HashSet::new();
    for letter in word.chars() {
        if !char_set.insert(letter) {
            return None;
        }
        output |= map.get(&letter).unwrap();
    }
    Some(output)
}

fn get_word_set(wordset: &mut HashMap<u32, String>) -> Option<HashSet<String>> {
    let mut selected_keys = HashSet::new();
    let mut output = HashSet::new();
    for (k, v) in wordset.iter() {
        if output.len() == 0 {
            output.insert(v.to_owned());
            selected_keys.insert(k.to_owned());
        } else if output.len() < 5 {
            let mut number = 0;
            for x in selected_keys.clone().iter() {
                number = k & x;
                if number != 0 {
                    break;
                };
            }
            if number == 0 {
                output.insert(v.to_owned());
                selected_keys.insert(k.to_owned());
            }
        } else {
            break;
        }
    }
    if output.len() == 5 {
        selected_keys.iter().for_each(|x| {
            wordset.remove(x);
        });
        return Some(output);
    }
    None
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

    let mut word_set: HashMap<u32, String> = HashMap::new();
    let mut solutions: Vec<Vec<String>> = Vec::new();

    let path = Path::new("sgb-words.txt");
    let file = File::open(path).expect("uh-oh");
    let reader = BufReader::new(file);

    println!("Building set of words with unique letters");
    let wordset_start = Instant::now();

    for line in reader.lines() {
        let word = line.unwrap();
        let word_as_bits = word_to_bits(&word, &letters_to_binary);
        if word_as_bits.is_some() {
            word_set.insert(word_as_bits.unwrap(), word);
        }
    }
    let wordset_end = wordset_start.elapsed().as_millis();
    println!("Built set in {}ms", wordset_end);

    println!("Finding solutions");
    let solution_start = Instant::now();

    while word_set.len() > 0 {
        let words = get_word_set(&mut word_set);
        if words.is_none() {
            break;
        }
        let list: Vec<String> = words.unwrap().into_iter().collect();
        solutions.push(list);
    }
    let solution_end = solution_start.elapsed().as_millis();
    println!("Found {} solutions in {}ms", solutions.len(), solution_end);
}
