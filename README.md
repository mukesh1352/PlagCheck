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

# Example output
```bash

Enter the path of the file to be compared:
/Users/mukesh/Documents/NNDL_Assignment-main/CB.EN.U4CSE22531.ipynb
Enter the comparison directory:
/Users/mukesh/Documents/NNDL_Assignment-main
Comparing file: /Users/mukesh/Documents/NNDL_Assignment-main/CB.EN.U4CSE22531.ipynb
With directory: /Users/mukesh/Documents/NNDL_Assignment-main

 Matching Report:

 File: CB.EN.U4CSE22365.ipynb         Match: 52.43%
 File: CB.EN.U4CSE22135_perceptron.ipynb Match: 43.75%
 File: CBENU4CSE22140.ipynb           Match: 41.61%
 File: CB.EN.U4CSE22324_NNDL.ipynb    Match: 46.43%
 File: CB.EN.U4CSE22037.ipynb         Match: 42.31%
 File: CB.EN.U4CSE22336.ipynb         Match: 42.62%
 File: CB.EN.U4CSE22310.ipynb         Match: 41.46%
 File: CB.EN.U4CSE22432.ipynb         Match: 21.90%
 File: CB.EN.U4CSE22312.ipynb         Match: 41.46%
 File: CB.EN.U4ECE22265.ipynb         Match: 39.88%
 File: CB.EN.U4ECE21041.ipynb         Match: 18.64%
 File: CB.EN.U4CSE22514.ipynb         Match: 39.88%
 File: CB.EN.U4CSE22453.ipynb         Match: 46.94%
 File: CB.EN.U4CSE22216.ipynb         Match: 100.00%
 File: CB.EN.U4CSE22503_git.ipynb     Match: 39.88%
 File: CB_EN_U4CSE22518.ipynb         Match: 27.35%
 File: CB.EN.U4CSE22018.ipynb         Match: 42.31%
 File: CB.EN.U4CSE22350.ipynb         Match: 90.91%
 File: CB.EN.U4CSE22451.ipynb         Match: 46.94%
 File: CB.EN.U4CSE22027.ipynb         Match: 42.31%
 File: CB.EN.U4CSE22531.ipynb         Match: 100.00%
 File: CB.EN.U4ECE22014.ipynb         Match: 16.10%
 File: CB.EN.U4CSE22515.ipynb         Match: 39.88%
 File: CB_EN_U4CSE22325.ipynb         Match: 23.85%
 File: CB.EN.U4CSE22505.ipynb         Match: 100.00%
 File: CB.EN.U4CSE22303.ipynb         Match: 40.52%
 File: CB.EN.U4CSE22311.ipynb         Match: 46.43%
 File: CB.EN.U4CSE22021.ipynb         Match: 39.88%
 File: CB.EN.U4CSE22250.ipynb         Match: 43.85%
 File: CBENU4CSE22148.ipynb           Match: 40.79%
 File: CB.EN.U4CSE22425.ipynb         Match: 23.01%
 File: CB.EN.U4ECE21159.ipynb         Match: 21.11%

 Completed in: 47.11ms (hh:mm:ss.ms)

 Success.
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