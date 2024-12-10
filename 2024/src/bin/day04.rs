use aoc2024::read_lines;

fn search_word(
    input: &Vec<Vec<char>>,
    word: &str,
    position: (usize, usize),
    direction: (i32, i32),
) -> bool {
    for (k, c) in word.char_indices() {
        let x = position.0 as i32 + k as i32 * direction.0;
        let y = position.1 as i32 + k as i32 * direction.1;

        if x < 0 || y < 0 || x as usize >= input.len() || y as usize >= input[x as usize].len() {
            return false;
        }

        if c != input[x as usize][y as usize] {
            return false;
        }
    }

    true
}

fn count_xmas(input: &Vec<Vec<char>>) -> usize {
    let directions = vec![
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut total = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            for direction in &directions {
                if search_word(input, "XMAS", (i, j), (direction[0], direction[1])) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn count_crosses(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for i in 1..(input.len() - 1) {
        for j in 1..(input[i].len() - 1) {
            if input[i][j] != 'A' {
                continue;
            }

            let diagonals = (
                input[i - 1][j - 1],
                input[i - 1][j + 1],
                input[i + 1][j - 1],
                input[i + 1][j + 1],
            );

            let matches = match diagonals {
                ('M', 'M', 'S', 'S') => true,
                ('S', 'M', 'S', 'M') => true,
                ('S', 'S', 'M', 'M') => true,
                ('M', 'S', 'M', 'S') => true,
                _ => false,
            };

            if matches {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    let lines = read_lines();
    let input: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();

    println!("Part 1 = {}", count_xmas(&input));
    println!("Part 1 = {}", count_crosses(&input));
}
