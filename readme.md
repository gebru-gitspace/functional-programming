# Rust Functional Programming Concepts

A compact Rust project demonstrating **functional programming paradigms** with idiomatic Rust code. This project emphasizes pure functions, closures, recursion, composition, pattern matching, the builder pattern and a file parser.

This repository contains two main parts:

1. **Functional Programming Concepts** (`concepts.rs`)  
2. **Text Analyzer** (`file_parser.rs`)

---

## Functional Programming concepts

- **Pure Functions:** Functions with no side-effects and predictable outputs.
- **Lambda & Closures:** Anonymous functions and closures capturing the environment.
- **Map, Filter, and Fold:** Functional transformations and reductions using iterators.
- **Recursion:** Example of factorial computation.
- **Function Composition & Currying:** Combining functions and returning partial functions.
- **Partial Application:** Fixing arguments to create specialized functions.
- **Pattern Matching & Enums:** Algebraic data types and `match`-based evaluation.
- **Builder Pattern:** Immutable builder pattern for struct construction.
- **Option Handling:** Safe handling of potentially missing data.


---
## 2. Text Analyzer / File Parser (`file_parser.rs`)

A command-line program that reads a text file and analyzes **word frequency** using **functional programming techniques**.

### Features

- **Count total words and unique words.**
- **Identify the most common word.**
- **Supports optional filters using closures:**
  - `--min-length N` : Only include words longer than `N`.
  - `--starts-with C` : Only include words starting with character `C`.
- **Text normalization:** converts all words to lowercase and removes punctuation.
- **Functional approach:** uses iterators, closures, and combinators, avoiding mutable loops.

---

## Project Structure

```text
functional-programming/
│
├── Cargo.toml                # Rust project manifest
├── src/
│   ├── main.rs               # Optional main script (could be empty or demo entry)
│   └── bin/
│       ├── concepts.rs       # Demonstrates functional programming concepts
│       └── file_parser.rs    # CLI text analyzer using functional programming
└── README.md                 # Project documentation


### Run the Functional Programming Examples

```bash
cargo run --bin concepts
```
Run the analzer
```
cargo run --bin file_parser -- <filename> [--min-length N] [--starts-with C]
```