//! Functional Text Analyzer in Rust
//! 
//! Reads a text file, counts word frequencies, and displays statistics.
//! Demonstrates functional programming in Rust using iterators, closures, and combinators.
//! Supports dynamic filtering: minimum word length and words starting with a specific character.

/// September 2025

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

/// Configuration for analysis
struct Config {
    file_path: String,
    min_length: Option<usize>,
    starts_with: Option<char>,
}

impl Config {
    /// Parse CLI arguments into a Config
    fn from_args() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err(format!(
                "Usage: {} <file_path> [--min-length N] [--starts-with C]",
                args[0]
            ));
        }

        let file_path = args[1].clone();
        let mut min_length: Option<usize> = None;
        let mut starts_with: Option<char> = None;

        // let mut i = 2;
        // while i < args.len() {
        //     match args[i].as_str() {
        //         "--min-length" => {
        //             if i + 1 < args.len() {
        //                 min_length = args[i + 1].parse().ok();
        //                 i += 1;
        //             }
        //         }
        //         "--starts-with" => {
        //             if i + 1 < args.len() {
        //                 starts_with = args[i + 1].chars().next();
        //                 i += 1;
        //             }
        //         }
        //         _ => {}
        //     }
        //     i += 1;
        // }

         // Parse cli flags
        let mut iter = args.iter().skip(2);
        while let Some(flag) = iter.next() {
            match flag.as_str() {
                "--min-length" => {
                    if let Some(n) = iter.next() {
                        min_length = n.parse::<usize>().ok();
                    }
                }
                "--starts-with" => {
                    if let Some(c) = iter.next() {
                        starts_with = c.chars().next();
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            file_path,
            min_length,
            starts_with,
        })
    }
}

/// Reads a file into a String
fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("Error reading file {}: {}", path, err))
}

/// Clean a word: keep only alphanumeric characters, lowercase it
fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase()) // handles Unicode case folding
        .collect()
}

/// Analyze text and count word frequencies functionally
fn analyze_text(text: &str, config: &Config) -> HashMap<String, usize> {
    // Filtering closure
    let filter = |word: &str| {
        let long_enough = config.min_length.map_or(true, |n| word.len() >= n);
        let starts_correct = config.starts_with.map_or(true, |c| word.starts_with(c));
        long_enough && starts_correct
    };

    text.split_whitespace()
        .map(clean_word)
        // .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        // .map(|w| w.to_lowercase())
        .filter(|w| !w.is_empty())
        .filter(|w| filter(w))
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

/// Display statistics of word frequencies
fn display_stats(freqs: &HashMap<String, usize>) {
    let total_words: usize = freqs.values().sum();
    let unique_words = freqs.len();
    let most_common = freqs.iter().max_by_key(|&(_, count)| count);

    println!("Total words: {}", total_words);
    println!("Unique words: {}", unique_words);

    if let Some((word, count)) = most_common {
        println!("Most common word: '{}' ({} occurrences)", word, count);
    } else {
        println!("No words found after filtering.");
    }
    //   println!("\n--- Word Frequencies ---");
    // for (word, count) in freqs {
    //     println!("{}: {}", word, count);
    // }
}

fn main() {
    let config = match Config::from_args() {
        Ok(cfg) => cfg,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let content = match read_file(&config.file_path) {
        Ok(txt) => txt,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let freqs = analyze_text(&content, &config);
    display_stats(&freqs);
}
