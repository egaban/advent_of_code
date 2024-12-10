use std::fmt::Debug;
use std::io::Read;
use std::{io::stdin, str::FromStr};

pub fn read_lines() -> Vec<String> {
    let mut result = Vec::new();

    for line in stdin().lines() {
        if let Err(e) = &line {
            println!("Error {}", e);
        }

        result.push(line.ok().unwrap());
    }

    result
}

pub fn read_input() -> String {
    let mut line = String::new();
    stdin()
        .read_to_string(&mut line)
        .expect("Failed to read input");
    line
}

pub fn split_line<T>(line: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line.split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
