fn extract_range(text: &str) -> (i64, i64) {
    let (start, end) = text.split_once("-").expect("Invalid range format");
    (
        start.parse().expect("Invalid start number"),
        end.parse().expect("Invalid end number"),
    )
}

fn num_digits(number: i64) -> usize {
    if number == 0 {
        return 1;
    }
    number.ilog10() as usize + 1
}

/// Given n, returns 10^n
fn pow10(n: usize) -> i64 {
    10i64.pow(n as u32)
}

/// Checks whether n repeats k times the same pattern.
fn forms_repeated_pattern(mut number: i64, k: usize) -> bool {
    let n = num_digits(number);
    if n == k || n % k != 0 {
        return false;
    }

    let base = pow10(k);
    let pattern = number % base;

    loop {
        number /= base;
        if number % base != pattern {
            return false;
        }
        if number == pattern {
            return true;
        }
    }
}

fn is_part1_invalid(number: i64) -> bool {
    let n = num_digits(number);

    // Odd number of digits, can't be repeated twice.
    if n % 2 != 0 {
        return false;
    }

    forms_repeated_pattern(number, n / 2)
}

fn is_part2_invalid(number: i64) -> bool {
    let n = num_digits(number);

    for k in 0..(n / 2) {
        if forms_repeated_pattern(number, k + 1) {
            return true;
        }
    }

    false
}

fn part1(ranges: &[(i64, i64)]) -> i64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| is_part1_invalid(i))
        .sum()
}

fn part2(ranges: &[(i64, i64)]) -> i64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| is_part2_invalid(i))
        .sum()
}

fn main() {
    let input = aoc::read_input("inputs/02.txt");
    let ranges: Vec<_> = input.trim().split(",").map(extract_range).collect();

    println!("Part 1 = {}", part1(&ranges));
    println!("Part 2 = {}", part2(&ranges));
}
