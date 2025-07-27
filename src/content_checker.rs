use rayon::prelude::*;
use similar::{Algorithm, DiffTag, TextDiff};
use std::time::Instant;
use std::fs;

pub fn content_checker1(pathname: &str, output_path: &str) {
   let start_time = Instant::now();
    let input_content = match fs::read_to_string(pathname) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let entries = match fs::read_dir(output_path) {
        Ok(e) => e.collect::<Result<Vec<_>, _>>().unwrap_or_default(),
        Err(e) => {
            eprintln!(" Error reading output directory: {}", e);
            return;
        }
    };

    println!("\n🧾 Matching Report:\n");

    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        if path.is_file() {
            if let Ok(comp_content) = fs::read_to_string(&path) {
                let similarity = calculate_similarity(&input_content, &comp_content);
                println!(
                    "📄 File: {:<30} Match: {:.2}%",
                    path.file_name().unwrap().to_string_lossy(),
                    similarity
                );
            }
        }
    });
    let elapsed = start_time.elapsed();
    println!("\n Completed in: {:.2?} (hh:mm:ss.ms)",elapsed);
}

// Helper to trim and normalize whitespace
fn normalize_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(|line| line.trim()) // trim whitespace from each line
        .filter(|line| !line.is_empty()) // skip empty lines (optional)
        .map(|line| line.to_string())
        .collect()
}

fn calculate_similarity(text1: &str, text2: &str) -> f64 {
    let norm1 = normalize_lines(text1).join("\n");
    let norm2 = normalize_lines(text2).join("\n");

    if norm1 == norm2 {
        return 100.0;
    }

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .diff_lines(&norm1, &norm2);

    let total_lines = diff
        .ops()
        .iter()
        .map(|op| op.new_range().len())
        .sum::<usize>();

    let matched_lines = diff
        .ops()
        .iter()
        .filter(|op| op.tag() == DiffTag::Equal)
        .map(|op| op.new_range().len())
        .sum::<usize>();

    if total_lines == 0 {
        0.0
    } else {
        (matched_lines as f64 / total_lines as f64) * 100.0
    }
}
