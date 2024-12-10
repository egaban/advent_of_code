use aoc2024::{read_lines, split_line};

fn is_safe(report: &Vec<i32>) -> bool {
    let sorted = report.is_sorted() || report.iter().rev().is_sorted();
    sorted
        && report.windows(2).all(|pair| {
            let diff = (pair[0] - pair[1]).abs();
            diff >= 1 && diff <= 3
        })
}

fn can_be_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let modified_report = report[0..i]
            .iter()
            .chain(&report[i + 1..report.len()])
            .copied()
            .collect::<Vec<_>>();

        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn main() {
    let lines = read_lines();

    let mut num_safe_reports = 0;
    let mut num_could_be_safe = 0;
    for line in lines {
        let report = split_line(&line);
        if is_safe(&report) {
            num_safe_reports += 1;
        }

        if can_be_safe(&report) {
            num_could_be_safe += 1;
        }
    }

    println!("Part 1 = {}", num_safe_reports);
    println!("Part 2 = {}", num_could_be_safe);
}
