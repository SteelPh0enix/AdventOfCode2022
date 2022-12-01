use std::fs;

const ELVES_SEPARATOR: i64 = -1;

fn main() {
    let mut elves = fs::read_to_string("./input")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap_or(ELVES_SEPARATOR))
        .collect::<Vec<i64>>()
        .split(|&weight| weight == ELVES_SEPARATOR)
        .map(|calories| calories.iter().fold(0u64, |acc, &x| acc + (x as u64)))
        .collect::<Vec<u64>>();

    elves.sort_unstable();

    let most_calories = elves.last().unwrap();
    println!("Elf carrying the most Calories carries {most_calories} Calories");

    let top_3_total_calories: u64 = elves.iter().rev().take(3).sum();
    println!("Top 3 elves have {top_3_total_calories} Calories");
}
