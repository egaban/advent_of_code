use std::{fs, io::BufRead};

/// Reads all lines of a file and returns a Vec of strings.
pub fn read_file(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}
