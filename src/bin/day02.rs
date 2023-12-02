use advent_of_code::read_file;

/// Red, green, blue
#[derive(Debug)]
struct Subset(u32, u32, u32);

struct Game {
    game_id: u32,
    subsets: Vec<Subset>,
}

fn main() {
    let lines = read_file("inputs/day02.input");

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for line in lines {
        let game = parse_line(&line);
        if game.valid() {
            part1_sum += game.game_id;
        }

        let mut minimum_subset = Subset::new();
        for subset in game.subsets {
            minimum_subset.0 = subset.0.max(minimum_subset.0);
            minimum_subset.1 = subset.1.max(minimum_subset.1);
            minimum_subset.2 = subset.2.max(minimum_subset.2);
        }
        let power = minimum_subset.0 * minimum_subset.1 * minimum_subset.2;
        part2_sum += power;
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}

fn parse_line(line: &str) -> Game {
    let split = line.split(":").collect::<Vec<&str>>();
    let game_id = parse_left_side(split[0]);
    let subsets = parse_right_side(split[1]);

    Game { game_id, subsets }
}

/// Returns game id
fn parse_left_side(left_side: &str) -> u32 {
    left_side.split_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .unwrap()
}

/// Returns the subsets
fn parse_right_side(right_side: &str) -> Vec<Subset> {
    let mut result = Vec::new();
    for subset in right_side.split(";") {
        result.push(parse_subset(subset));
    }
    result
}

fn parse_subset(subset: &str) -> Subset {
    let mut result = Subset::new();
    for entry in subset.split(",") {
        let split = entry.split_whitespace().collect::<Vec<&str>>();
        let amount = split[0].parse().unwrap();
        let color = split[1];

        match color {
            "red" => result.0 = amount,
            "green" => result.1 = amount,
            "blue" => result.2 = amount,
            _ => panic!("Unknown color: {}", color),
        }
    }

    result
}

impl Game {
    fn valid(&self) -> bool {
        for subset in &self.subsets {
            if !subset.valid() {
                return false;
            }
        }

        true
    }
}

impl Subset {
    fn new() -> Subset {
        Subset(0, 0, 0)
    }

    fn valid(&self) -> bool {
        self.0 <= 12 && self.1 <= 13 && self.2 <= 14
    }
}
