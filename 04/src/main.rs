use std::fs;
use std::ops::Range;

fn ranges_overlap_fully<T>(first: &Range<T>, second: &Range<T>) -> bool
where
    T: PartialOrd,
{
    (first.start <= second.start && first.end >= second.end)
        || (second.start <= first.start && second.end >= first.end)
}

fn ranges_overlap<T>(first: &Range<T>, second: &Range<T>) -> bool
where
    T: PartialOrd,
{
    (first.start <= second.end) && (second.start <= first.end)
}

type Sections = Range<i32>;
type Pair = (Sections, Sections);

fn parse_pair(pair: &str) -> Sections {
    let pair_split = pair.split_once('-').unwrap();
    (pair_split.0.parse::<i32>().unwrap())..(pair_split.1.parse::<i32>().unwrap())
}

fn parse_line(line: &str) -> Pair {
    let pair_split = line.split_once(',').unwrap();
    (parse_pair(pair_split.0), parse_pair(pair_split.1))
}

fn main() {
    let pairs = fs::read_to_string("./input")
        .unwrap()
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<Pair>>();

    let overlapping_pairs_amount = pairs
        .iter()
        .filter(|&(first, second)| ranges_overlap_fully(first, second))
        .count();
    println!("There are {overlapping_pairs_amount} overlapping pairs of elves");

    let partially_overlapping_pairs_amount = pairs
        .iter()
        .filter(|&(first, second)| ranges_overlap(first, second))
        .count();
    println!("There are {partially_overlapping_pairs_amount} partially overlapping pairs of elves");
}

#[cfg(test)]
mod tests {
    use crate::{ranges_overlap, ranges_overlap_fully};
    use std::ops::Range;

    #[test]
    fn ranges_overlap_fully_checks_overlap_correctly() {
        let test_range = 0..5;
        let fitting_ranges = [0..4, 1..5, 1..4, 0..5, 2..3, 4..4];
        let overlapping_ranges = [-1..6, -1..5, 0..6, -5..10];
        let non_overlapping_ranges = [1..10, -3..4, -10..2, 3..20];

        for range in &fitting_ranges {
            assert_eq!(ranges_overlap_fully(&test_range, range), true);
        }

        for range in &overlapping_ranges {
            assert_eq!(ranges_overlap_fully(&test_range, range), true);
        }

        for range in &non_overlapping_ranges {
            assert_eq!(ranges_overlap_fully(&test_range, range), false);
        }
    }

    #[test]
    fn ranges_overlap_checks_overlap_correctly() {
        let test_range = 0..5;
        let fully_overlapping_ranges = [
            0..4,
            1..5,
            1..4,
            0..5,
            2..3,
            4..4,
            -1..6,
            -1..5,
            0..6,
            -5..10,
        ];
        let partially_overlapping_ranges =
            [-1..3, -8..1, -3..0, 2..10, 3..8, 4..9, 1..12, -4..3, 5..12];
        let non_overlapping_ranges = [-4..-1, 8..10, -10..-2, 6..12];

        for range in &fully_overlapping_ranges {
            assert_eq!(ranges_overlap(&test_range, range), true);
        }

        for range in &partially_overlapping_ranges {
            assert_eq!(ranges_overlap(&test_range, range), true);
        }

        for range in &non_overlapping_ranges {
            assert_eq!(ranges_overlap(&test_range, range), false);
        }
    }
}
