use std::collections::HashMap;

use aoc2024::{read_lines, split_line};

fn build_count_map(list: &Vec<i32>) -> HashMap<i32, i32> {
    list.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    })
}

fn main() {
    let lines = read_lines();
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|l| {
            let split = split_line::<i32>(l);
            (split[0], split[1])
        })
        .unzip();

    list1.sort();
    list2.sort();

    let mut abs_dif = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        abs_dif += (a - b).abs();
    }

    let count_map = build_count_map(&list2);

    let mut similarity = 0;
    for num in list1 {
        similarity += num * count_map.get(&num).unwrap_or(&0);
    }

    println!("Part 1 = {}", abs_dif);
    println!("Part 2 = {}", similarity);
}
