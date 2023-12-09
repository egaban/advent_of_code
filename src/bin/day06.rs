use advent_of_code::read_file;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

fn main() {
    let lines = read_file("inputs/day06.input");
    let races = parse(&lines);

    let mut result = 1;
    for race in races {
        result *= race.ways_can_win();
    }
    println!("Part 1 = {}", result);

    let race = parse_part2(&lines);
    println!("Part 2 = {}", race.ways_can_win());
}

fn parse(lines: &Vec<String>) -> Vec<Race> {
    let mut result = Vec::new();

    let times = lines[0].trim().split_whitespace().collect::<Vec<&str>>();
    let distances = lines[1].trim().split_whitespace().collect::<Vec<&str>>();

    for i in 1..times.len() {
        let race = Race {
            time: times[i].parse().unwrap(),
            record: distances[i].parse().unwrap(),
        };
        result.push(race);
    }

    result
}

fn parse_part2(lines: &Vec<String>) -> Race {
    let time = lines[0].trim().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat();
    let distance = lines[1].trim().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat();

    Race {
        time: time.parse().unwrap(),
        record: distance.parse().unwrap(),
    }
}

impl Race {
    /// We only need to consider 1..n/2, because the "resulting array" is symmetric.
    /// That way, we have a strictly increasing function, and we can use binary search.
    fn ways_can_win(&self) -> usize {
        // n is the same as 0, so subtract 1.
        let half = ((self.time - 1) as f64 / 2.0).ceil() as usize;
        let mut left = 1;
        let mut right = half;

        // Finds the first winning index.
        while left < right {
            let mid = (left + right) / 2;
            if self.wins(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        let mut result = 2 * (half - left + 1);
        // We added n/2 twice, so we need to subtract one if n is even.
        if self.time % 2 == 0 {
            result -= 1;
        }
        result
    }

    /// Checks if wins by holding until the given time.
    fn wins(&self, hold_until: usize) -> bool {
        hold_until * (self.time - hold_until) > self.record
    }
}
