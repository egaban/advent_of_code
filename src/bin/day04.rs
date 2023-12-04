use std::collections::HashSet;

use advent_of_code::read_file;

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    have: HashSet<usize>,
}

fn main() {
    let lines = read_file("inputs/day04.input");
    let mut num_cards = vec![1; lines.len()];

    let mut sum = 0;
    for line in lines {
        let card = Card::parse(&line);
        sum += card.points();

        if card.matches() > 0 {
            let amount_cur_card = num_cards[card.id - 1];
            add_cards(&mut num_cards, card.id, card.matches(), amount_cur_card);
        }
    }

    println!("points: {}", sum);
    println!("total cards: {}", num_cards.iter().sum::<usize>());
}

fn add_cards(num_cards: &mut Vec<usize>, cur_card: usize, matches: usize, amount_cur_card: usize) {
    for i in cur_card..cur_card + matches {
        num_cards[i] += amount_cur_card;
    }
}

impl Card {
    fn parse(line: &str) -> Self {
        let split: Vec<&str> = line.trim().split(":").collect();
        let id = Self::parse_id(split[0]);
        let split: Vec<&str> = split[1].split("|").collect();
        let winning = Self::parse_winning(split[0]);
        let have = Self::parse_have(split[1]);

        Self { id, have, winning }
    }

    fn parse_id(line: &str) -> usize {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }

    fn parse_have(line: &str) -> HashSet<usize> {
        let mut result = HashSet::new();
        for num in line.split_whitespace() {
            result.insert(num.parse::<usize>().unwrap());
        }
        result
    }

    fn parse_winning(line: &str) -> HashSet<usize> {
        let mut result = HashSet::new();
        for num in line.split_whitespace() {
            result.insert(num.parse::<usize>().unwrap());
        }
        result
    }

    fn matches(&self) -> usize {
        self.winning
            .iter()
            .filter(|x| self.have.contains(x))
            .count()
    }

    fn points(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }

        2usize.pow((matches - 1) as u32)
    }
}
