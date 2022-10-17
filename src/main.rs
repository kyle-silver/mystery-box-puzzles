use std::fs::File;
use std::str::Chars;
use std::{collections::HashSet};
use std::io::Write;

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


const corpus: &str = include_str!("res/scrabble_dictionary_lowercase.txt");
const combinations: &str = include_str!("res/combinations.txt");

fn is_subset_of(chars: &[char], word: &[char]) -> bool {
    for letter in word {
        if let Err(_) = chars.binary_search(letter) {
            return false;
        }
    }
    return true;
}

fn all_words<'a>(chars: &[char], words: &'a [(&str, Vec<char>)]) -> Vec<&'a str> {
    words.iter()
        .filter_map(|(word, letters)| {
            if is_subset_of(chars, letters) {
                Some(*word)
            } else {
                None
            }
        })
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let words = corpus.lines()
    .map(|line| {
        let char_set = HashSet::<_>::from_iter(line.chars());
        (line, char_set)
    })
    .filter(|(word, _)| (4..=8).contains(&word.len()))
    .filter(|(word, letters)| word.len() == letters.len())
    .map(|(word, letters)| (word, letters.into_iter().collect_vec()))
    .collect_vec();
    
    println!("All words parsed");
    
    let combos = combinations.lines()
        .map(&str::chars)
        .map(Chars::collect_vec)
        .map(|mut letters| {
            letters.sort_unstable();
            letters
        })
        .enumerate()
        .collect_vec();
    
    println!("All combinations generated");

    let total = combos.len();

    let puzzles: Vec<_> = combos
        .par_iter()
        .filter_map(|(i, combo)| {
            let found_words = all_words(combo, &words);
            if i % 1000 == 0 {
                let jumble: String = combo.iter().collect();
                println!("[{i} / {total}]: {jumble:?}\t{found_words:?}");
            }
            if found_words.len() >= 8 {
                let jumble: String = combo.iter().collect();
                Some((jumble, found_words))
            } else {
                None
            }
        })
        .collect();

    println!("Puzzles generated: {}", puzzles.len());
    println!("Writing puzzles");

    let mut output = File::create("output.json")?;
    writeln!(output, "[")?;
    for (puzzle, solutions) in puzzles {
        if let Err(e) = writeln!(output, "{{ \"input\": \"{puzzle}\", \"solutions\": {solutions:?} }},") {
            eprintln!("Couldn't write to file: {e}");
        }
    }
    writeln!(output, "]")?;
    Ok(())
}
