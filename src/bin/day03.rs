use advent_of_code::read_file;
use regex::Regex;

struct Range(usize, usize);

fn main() {
    let lines = read_file("inputs/day03.input");

    let mut sum = 0;
    let mut sum_part2 = 0;
    for cur_line in 0..lines.len() {
        let num_regex = Regex::new("\\d+").unwrap();
        let asterisk_regex = Regex::new("\\*").unwrap();

        for m in num_regex.find_iter(&lines[cur_line]) {
            let range = get_range(&lines, m.start(), m.end());
            if has_symbol(&lines, cur_line, range) {
                sum += m.as_str().parse::<i32>().unwrap();
            }
        }

        for m in asterisk_regex.find_iter(&lines[cur_line]) {
            let range = get_range(&lines, m.start(), m.end());
            let numbers = adjacent_numbers(&lines, cur_line, &range);

            if numbers.len() == 2 {
                sum_part2 += numbers[0] * numbers[1];
            }
        }
    }

    println!("Part 1 = {}", sum);
    println!("Part 2 = {}", sum_part2);
}

fn has_symbol(lines: &Vec<String>, cur_line: usize, range: Range) -> bool {
    if cur_line > 0 && check_line(&lines[cur_line - 1], &range) {
        return true;
    }

    if cur_line < lines.len() - 1 && check_line(&lines[cur_line + 1], &range) {
        return true;
    }

    check_line(&lines[cur_line], &range)
}

fn check_line(line: &str, range: &Range) -> bool {
    for i in range.0..range.1 {
        let char = line.chars().nth(i).unwrap();
        if !char.is_numeric() && char != '.' {
            return true;
        }
    }
    false
}

fn get_range(lines: &Vec<String>, start: usize, end: usize) -> Range {
    let mut start = start;
    let mut end = end;

    if start > 0 {
        start -= 1;
    }

    if end < lines[0].len() {
        end += 1;
    }

    Range(start, end)
}

fn adjacent_numbers(lines: &Vec<String>, cur_line: usize, range: &Range) -> Vec<usize> {
    let mut result = Vec::new();

    if cur_line > 0 {
        result.append(&mut numbers_in_range(&lines[cur_line - 1], range));
    }

    if cur_line < lines.len() - 1 {
        result.append(&mut numbers_in_range(&lines[cur_line + 1], range));
    }

    result.append(&mut numbers_in_range(&lines[cur_line], range));

    result
}

fn numbers_in_range(line: &str, range: &Range) -> Vec<usize> {
    let num_regex = Regex::new("\\d+").unwrap();
    let mut result = Vec::new();

    for m in num_regex.find_iter(line) {
        if m.start() <= (range.1 - 1) && range.0 <= (m.end() - 1) {
            result.push(m.as_str().parse::<usize>().unwrap());
        }
    }

    result
}
