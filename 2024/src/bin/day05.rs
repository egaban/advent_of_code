use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

struct Instance {
    rules: HashMap<usize, Vec<usize>>,
    orderings: Vec<Vec<usize>>,
}

fn main() {
    let instance = Instance::read();

    let mut part1 = 0;
    let mut invalids = Vec::new();
    for ordering in &instance.orderings {
        if instance.check_ordering(&ordering) {
            part1 += ordering[ordering.len() / 2];
        } else {
            invalids.push(ordering.clone());
        }
    }

    invalids.iter_mut().for_each(|x| {
        x.sort_by(|x, y| {
            instance
                .rules
                .get(x)
                .map(|rules| rules.contains(y))
                .unwrap_or(false)
                .then(|| std::cmp::Ordering::Less)
                .unwrap_or(std::cmp::Ordering::Greater)
        })
    });

    let mut part2 = 0;
    for reordered in invalids {
        part2 += reordered[reordered.len() / 2];
    }

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}

impl Instance {
    fn read() -> Self {
        let mut buffer = BufReader::new(std::io::stdin());
        let mut rules = HashMap::new();
        let mut orderings = Vec::new();

        let mut line = String::new();
        while let Ok(_) = buffer.read_line(&mut line) {
            if line.trim().is_empty() {
                break;
            }

            let rule: Vec<usize> = line
                .trim()
                .split("|")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            rules.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);

            line.clear();
        }

        while let Ok(n) = buffer.read_line(&mut line) {
            if n == 0 {
                break;
            }

            let ordering: Vec<usize> = line
                .trim()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            orderings.push(ordering);
            line.clear();
        }

        Self { rules, orderings }
    }

    fn check_ordering(&self, ordering: &Vec<usize>) -> bool {
        let mut used = HashSet::new();

        for number in ordering {
            used.insert(number);
            let rules = self.rules.get(number);

            if let None = rules {
                continue;
            }

            for number in rules.unwrap() {
                if used.contains(number) {
                    return false;
                }
            }
        }

        true
    }
}
