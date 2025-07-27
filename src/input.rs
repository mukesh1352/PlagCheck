pub fn file_inputter(pathname: &str, output_path: &str) -> bool {
    if !pathname.is_empty() && !output_path.is_empty() {
        println!("Comparing file: {}\nWith directory: {}", pathname, output_path);
        true
    } else {
        false
    }
}
