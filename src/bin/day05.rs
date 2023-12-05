use std::{
    fs,
    io::{BufRead, BufReader},
    sync::Arc,
};

#[derive(Debug)]
struct Range {
    mapped: usize,
    start: usize,
    len: usize,
}

struct Seed {
    start: usize,
    len: usize,
}

struct Instance {
    seeds: Vec<Seed>,
    maps: Vec<Vec<Range>>,
}

fn main() {
    let f = fs::File::open("inputs/day05.input").unwrap();
    let mut reader = std::io::BufReader::new(f);

    let instance = Instance {
        seeds: parse_seeds(&mut reader),
        maps: parse_maps(&mut reader),
    };
    let instance = Arc::new(instance);

    let mut min_location = usize::MAX;
    for seed in &instance.seeds {
        min_location = location(&instance, seed.start).min(min_location);
        min_location = location(&instance, seed.len).min(min_location);
    }

    println!("Part 1 = {}", min_location);

    // Now brute force the seeds :(
    let mut handles = Vec::new();
    for i in 0..instance.seeds.len() {
        let instance_clone = instance.clone();
        let handle = std::thread::spawn(move || process_seed_range(instance_clone, i));
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        min_location = min_location.min(result);
    }

    println!("Part 2 = {}", min_location);
}

fn location(instance: &Instance, seed: usize) -> usize {
    let mut location = seed;
    for i in 0..7 {
        location = check_map(&instance.maps[i], location);
    }
    location
}

fn parse_seeds(reader: &mut BufReader<fs::File>) -> Vec<Seed> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut _discard = String::new();
    reader.read_line(&mut _discard).unwrap();

    let right_side = line.trim().split(":").into_iter().last().unwrap();
    right_side
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter_map(|s| {
            if let [start, len] = s {
                Some(Seed {
                    start: start.parse().unwrap(),
                    len: len.parse().unwrap(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn parse_maps(reader: &mut BufReader<fs::File>) -> Vec<Vec<Range>> {
    let mut result = Vec::new();
    for _ in 0..7 {
        let map = parse_map(reader);
        result.push(map);
    }
    result
}

fn parse_map(reader: &mut BufReader<fs::File>) -> Vec<Range> {
    let mut result = Vec::new();

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.trim().split_whitespace();
        let range = Range {
            mapped: parts.next().unwrap().parse().unwrap(),
            start: parts.next().unwrap().parse().unwrap(),
            len: parts.next().unwrap().parse().unwrap(),
        };

        result.push(range);
    }

    // Sorts the ranges by start position
    result.sort_by(|a, b| a.start.cmp(&b.start));
    result
}

fn check_map(map: &Vec<Range>, key: usize) -> usize {
    for range in map {
        if range.start <= key && key < range.start + range.len {
            let diff = key - range.start;
            return range.mapped + diff;
        }
    }

    key
}

fn process_seed_range(instance: Arc<Instance>, seed_index: usize) -> usize {
    let seed = &instance.seeds[seed_index];
    let mut min_location = usize::MAX;
    for i in seed.start..seed.start + seed.len {
        let location = location(&instance, i);
        min_location = min_location.min(location);
    }

    min_location
}
