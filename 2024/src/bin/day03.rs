use aoc2024::read_input;
use regex::Regex;

fn process_instructions(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to create regex");

    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a.trim().parse::<i32>().unwrap();
        let b = b.trim().parse::<i32>().unwrap();
        total += a * b;
    }

    total
}

fn extract_valid_instructions(input: &str) -> String {
    let re = Regex::new(r"don't\(\)[\s\S]*?do\(\)").expect("Failed to create regex");
    re.replace_all(input, "").to_string()
}

fn main() {
    let input = read_input();
    let filtered = extract_valid_instructions(&input);

    println!("Part 1 = {}", process_instructions(&input));
    println!("Part 2 = {}", process_instructions(&filtered));
}
