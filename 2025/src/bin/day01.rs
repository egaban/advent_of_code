use regex::Regex;

fn part1(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"^(L|R)(\d+)$").unwrap();
    let mut result = 0;
    let mut dial_position = 50;

    for line in lines {
        if let Some(c) = re.captures(line) {
            let direction = &c[1];
            let moves = c[2].parse::<i32>().unwrap();

            let moves = match direction {
                "L" => -moves,
                "R" => moves,
                _ => panic!("Invalid direction {}", direction),
            };

            dial_position = (dial_position + moves).rem_euclid(100);

            if dial_position == 0 {
                result += 1;
            }
        }
    }

    result
}

fn part2(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"^(L|R)(\d+)$").unwrap();
    let mut result = 0;
    let mut dial_position: i32 = 50;

    for line in lines {
        if let Some(c) = re.captures(line) {
            let direction = &c[1];
            let mut moves = c[2].parse::<i32>().unwrap();

            let direction = match direction {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction {}", direction),
            };

            while moves > 0 {
                dial_position = (dial_position + direction).rem_euclid(100);
                if dial_position == 0 {
                    result += 1;
                }
                moves -= 1;
            }
        }
    }

    result
}

fn main() {
    let input = aoc::read_input_lines("inputs/01.txt");

    println!("Part 1 = {}", part1(&input));
    println!("Part 2 = {}", part2(&input));
}
