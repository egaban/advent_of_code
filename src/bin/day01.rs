use advent_of_code::read_file;

const DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn parse_digit(number: &str) -> u32 {
    if let Ok(number) = number.parse::<u32>() {
        return number;
    }

    match number {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    }
}

fn first_digit(line: &str) -> u32 {
    let mut first_position: Option<usize> = None;
    let mut first_digit: Option<u32> = None;

    for &digit in DIGITS.iter() {
        if let Some(position) = line.find(digit) {
            match first_position {
                Some(fp) => {
                    if position < fp {
                        first_position = Some(position);
                        first_digit = Some(parse_digit(digit));
                    }
                }
                None => {
                    first_position = Some(position);
                    first_digit = Some(parse_digit(digit));
                }
            }
        }
    }

    first_digit.unwrap()
}

fn last_digit(line: &str) -> u32 {
    let mut last_position: Option<usize> = None;
    let mut last_digit: Option<u32> = None;

    for &digit in DIGITS.iter() {
        if let Some(position) = line.rfind(digit) {
            match last_position {
                Some(lp) => {
                    if position > lp {
                        last_position = Some(position);
                        last_digit = Some(parse_digit(digit));
                    }
                }
                None => {
                    last_position = Some(position);
                    last_digit = Some(parse_digit(digit));
                }
            }
        }
    }

    last_digit.unwrap()
}

fn extract_num(line: &str) -> u32 {
    let first = first_digit(line);
    let last = last_digit(line);

    first * 10 + last
}

fn main() {
    let mut result = 0;

    for line in read_file("inputs/day01.input") {
        let num = extract_num(&line);
        println!("{} {}", line, num);
        result += num;
    }

    println!("{}", result);
}
