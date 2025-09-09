use std::collections::HashMap;
use std::env;
use std::fs;

/// Simple Text Analyzer
/// Reads a file, counts word frequencies, and supports optional filters:
/// --min-length N  : Only count words of length >= N
/// --starts-with C : Only count words starting with C

fn main() {
    // Collect CLI args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <filename> [--min-length N] [--starts-with C]");
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Error: could not read file {}", filename);
        std::process::exit(1);
    });

    // Default filter settings
    let mut min_length: Option<usize> = None;
    let mut starts_with: Option<char> = None;

    // Parse optional flags
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

    // Build filter closure
    let filter = |word: &str| {
        let long_enough = min_length.map_or(true, |n| word.len() >= n); // inclusive
        let starts_correct = starts_with.map_or(true, |c| word.starts_with(c));
        long_enough && starts_correct
    };

    // Split into words, normalize, apply filters, and count
    let mut freqs: HashMap<String, usize> = HashMap::new();
    content
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric())) // strip punctuation
        .map(|w| w.to_lowercase())
        .filter(|w| !w.is_empty())
        .filter(|w| filter(w))
        .for_each(|w| {
            *freqs.entry(w).or_insert(0) += 1;
        });

    // Total and unique word counts
    let total_words: usize = freqs.values().sum();
    let unique_words: usize = freqs.len();

    // Most common word
    let most_common = freqs.iter().max_by_key(|&(_, count)| count);

    // Output results
    println!("Total words: {}", total_words);
    println!("Unique words: {}", unique_words);
    if let Some((word, count)) = most_common {
        println!("Most common word: '{}' ({} times)", word, count);
    } else {
        println!("No words found after filtering.");
    }
}
