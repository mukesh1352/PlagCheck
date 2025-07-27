# Rust File Similarity Checker

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/) 

A fast, parallelized tool for comparing file similarity using line-based diff analysis.

## Features

-  Parallel processing with Rayon
-  Accurate similarity percentages using Myers diff algorithm
-  Line-level comparisons with whitespace normalization
-  Measures and reports total processing time
-  Compares one file against all files in a directory

## Installation

1. Ensure you have Rust installed (v1.70+ recommended)
2. Clone the repository:
```bash
git clone https://github.com/mukesh1352/PlagCheck.git
```
3. Build in release mode
```bash
cd PlagCheck
cargo build --release
```

## How to run
```bash
cargo run --release
```


 ### How it works
1.Reads the reference file line by line

2.Scans the target directory for all files

3.Uses parallel processing to compare each file:

4.Normalizes lines (trims whitespace, ignores empties)

5.Computes differences using Myers algorithm

6.Calculates similarity percentage

7.Displays results sorted by similarity

# CARGO.toml DEPENDENCIES
```bash
similar = "2.1"
rayon = "1.7"
blake = "2.0.2"
```
## PROJECT STRUCTURE
```bash
src/
├── main.rs              # CLI interface
├── input.rs             # File I/O operations
├── content_checker.rs   # Core comparison logic
├── utils.rs             # Helper functions
tests/                   # Unit tests
Cargo.toml               # Project configuration
```

## License
MIT License - See LICENSE for details.