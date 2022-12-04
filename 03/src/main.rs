use std::{collections::HashSet, fs, hash::Hash, str::FromStr};

// Every item type is identified by a single letter (case-sensitive)
// Single line contains list of items in one rucksack
// Every rucksack has 2 compartments, each containing the same amount of items
// Each half of input line is list of items in each compartment
// Every item type has priority, [a-z] => [1-26], [A-Z] => [27-52]

type Item = u8;

trait Weightable {
    fn weight(&self) -> u8;
}

impl Weightable for Item {
    fn weight(&self) -> u8 {
        if self.is_ascii_lowercase() {
            return self - b'a' + 1u8;
        }
        if self.is_ascii_uppercase() {
            return self - b'A' + 27u8;
        }
        return 0;
    }
}

struct Rucksack {
    compartments: [HashSet<Item>; 2],
}

impl Rucksack {
    fn shared_item(&self) -> Item {
        *self.compartments[0]
            .intersection(&self.compartments[1])
            .next()
            .unwrap()
    }

    fn content(&self) -> HashSet<Item> {
        let mut content = self.compartments[0].clone();
        content.extend(self.compartments[1].iter());
        content
    }

    fn find_badge(&self, second: &Rucksack, third: &Rucksack) -> Item {
        let first = self.content();
        let mut second = second.content();
        let mut third = third.content();

        let common: HashSet<Item> = first.iter().filter_map(|item| second.take(item)).collect();
        common
            .iter()
            .filter_map(|item| third.take(item))
            .next()
            .unwrap_or(0xFF)
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item_count = s.as_bytes().len() / 2;
        Ok(Rucksack {
            compartments: [
                HashSet::from_iter(
                    s.as_bytes()
                        .get(0..item_count)
                        .unwrap()
                        .to_owned()
                        .into_iter(),
                ),
                HashSet::from_iter(
                    s.as_bytes()
                        .get(item_count..(item_count * 2))
                        .unwrap()
                        .to_owned()
                        .into_iter(),
                ),
            ],
        })
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let rucksacks = input
        .lines()
        .map(|line| Rucksack::from_str(line).unwrap())
        .collect::<Vec<Rucksack>>();

    let shared_items_weight_sum: u32 = rucksacks
        .iter()
        .map(|sack| sack.shared_item().weight() as u32)
        .sum();

    println!("Sum of priorities of shared items is {shared_items_weight_sum}");

    // no `array_chunks` on stable Rust, rip
    let mut badges: Vec<Item> = Vec::new();
    let mut i = 0;
    while i < rucksacks.len() {
        let first_rucksack = &rucksacks[i];
        let second_rucksack = &rucksacks[i + 1usize];
        let third_rucksack = &rucksacks[i + 2usize];

        badges.push(first_rucksack.find_badge(second_rucksack, third_rucksack));
        i += 3;
    }

    let badges_sum: u32 = badges.iter().map(|&b| b.weight() as u32).sum();
    println!("{badges_sum}");
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr};

    use crate::{Rucksack, Weightable};

    #[test]
    fn rucksack_is_correctly_created_from_str() {
        let input = "qwertyuiopQWERTYUIOP";
        let sack = Rucksack::from_str(input).unwrap();

        assert_eq!(
            sack.compartments[0],
            HashSet::from_iter(b"qwertyuiop".to_owned().into_iter())
        );
        assert_eq!(
            sack.compartments[1],
            HashSet::from_iter(b"QWERTYUIOP".to_owned().into_iter())
        );
    }

    #[test]
    fn rucksack_detects_shared_element() {
        let input = "qweAwE";
        let sack = Rucksack::from_str(input).unwrap();

        assert_eq!(sack.shared_item(), b'w');
    }

    #[test]
    fn item_weight_is_correctly_calculated() {
        let items = b"abcxyzABCXYZ";
        let expected_weights = [
            1u8, 2u8, 3u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8, 50u8, 51u8, 52u8,
        ];

        items
            .iter()
            .zip(expected_weights.iter())
            .for_each(|(&item, &weight)| assert_eq!(item.weight(), weight));
    }

    #[test]
    fn finding_badges_works() {
        let rucksacks = ["qwertyQWENTY", "asdfthASDFGH", "axcvtnZXCVBN"]
            .map(|line| Rucksack::from_str(line).unwrap());

        assert_eq!(rucksacks[0].find_badge(&rucksacks[1], &rucksacks[2]), b't');
    }
}
