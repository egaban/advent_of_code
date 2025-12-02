use std::fs;

pub fn read_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|x| x.to_owned())
        .collect()
}
