use std::fs;

use regex::Regex;

type SupplyStorage = Vec<Vec<u8>>;

fn parse_supply_storage(lines: Vec<&str>) -> SupplyStorage {
    // columns have constant width, and there's always 4 characters per column (newline is omitted, hence +1)
    let columns_amount = (lines[0].bytes().len() + 1) / 4;
    let mut storage = vec![Vec::new(); columns_amount];

    // so we can just parse every 4th character to check if there's a crate on stack, or not
    for line in lines {
        for (column, crate_chunk) in line.as_bytes().chunks(4).enumerate() {
            let crate_name = crate_chunk[1];
            if crate_name != b' ' {
                storage[column].push(crate_name);
            }
        }
    }

    // we put crates in reverse order in our storage, so we gotta flip it
    for column in &mut storage {
        column.reverse();
    }

    storage
}

fn print_supply_storage(storage: &SupplyStorage) {
    let height = storage.iter().map(|column| column.len()).max().unwrap();
    for row in 0..height {
        let required_length = height - row;
        for column in storage {
            if column.len() >= required_length {
                let item = column[required_length - 1] as char;
                print!("[{item}] ");
            } else {
                print!("    ");
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct CratesTransfer {
    amount: usize,
    from: usize,
    to: usize,
}

fn move_crates(storage: &mut SupplyStorage, transfer: &CratesTransfer) {
    for _ in 0..transfer.amount {
        let crate_name = storage[transfer.from].pop().unwrap();
        storage[transfer.to].push(crate_name);
    }
}

fn move_crates_all_at_once(storage: &mut SupplyStorage, transfer: &CratesTransfer) {
    let height = storage[transfer.from].len();
    let remaining_height = height - transfer.amount;
    for i in 0..transfer.amount {
        let crate_name = storage[transfer.from][remaining_height + i];
        storage[transfer.to].push(crate_name);
    }
    storage[transfer.from].truncate(remaining_height);
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let storage_lines = &lines[..8];
    let mut storage = parse_supply_storage(storage_lines.to_vec());
    println!("Before:");
    print_supply_storage(&storage);

    // i've tested it. Manually. Once. Maybe twice. Trust me.
    let transfer_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let transfers = lines[10..]
        .iter()
        .map(|&line| {
            let captures = transfer_regex.captures(line).unwrap();
            CratesTransfer {
                amount: captures[1].parse::<usize>().unwrap(),
                from: captures[2].parse::<usize>().unwrap() - 1usize,
                to: captures[3].parse::<usize>().unwrap() - 1usize,
            }
        })
        .collect::<Vec<CratesTransfer>>();

    transfers.iter().for_each(|transfer| {
        move_crates_all_at_once(&mut storage, transfer);
    });

    println!("After:");
    print_supply_storage(&storage);
}

#[cfg(test)]
mod tests {
    use crate::{move_crates, parse_supply_storage, CratesTransfer, SupplyStorage, move_crates_all_at_once};

    #[test]
    fn supply_is_parsed_correctly() {
        let input_a = "[A]     [B]     [C]\n[D] [E] [F] [G] [H]\n";
        let expected_a: SupplyStorage = [
            [b'D', b'A'].to_vec(),
            [b'E'].to_vec(),
            [b'F', b'B'].to_vec(),
            [b'G'].to_vec(),
            [b'H', b'C'].to_vec(),
        ]
        .to_vec();

        assert_eq!(parse_supply_storage(input_a.lines().collect()), expected_a);
    }

    #[test]
    fn crates_are_transferred_correctly() {
        let mut storage: SupplyStorage = [
            [b'D', b'A'].to_vec(),
            [b'E'].to_vec(),
            [b'F', b'B'].to_vec(),
            [b'G'].to_vec(),
            [b'H', b'C'].to_vec(),
        ]
        .to_vec();

        let transfer = CratesTransfer {
            amount: 2,
            from: 2,
            to: 0,
        };

        let expected_storage: SupplyStorage = [
            [b'D', b'A', b'B', b'F'].to_vec(),
            [b'E'].to_vec(),
            [].to_vec(),
            [b'G'].to_vec(),
            [b'H', b'C'].to_vec(),
        ]
        .to_vec();

        move_crates(&mut storage, &transfer);
        assert_eq!(storage, expected_storage);
    }

    #[test]
    fn crates_are_transferred_correctly_all_at_once() {
        let mut storage: SupplyStorage = [
            [b'D', b'A'].to_vec(),
            [b'E'].to_vec(),
            [b'F', b'B'].to_vec(),
            [b'G'].to_vec(),
            [b'H', b'C'].to_vec(),
        ]
        .to_vec();

        let transfer = CratesTransfer {
            amount: 2,
            from: 2,
            to: 0,
        };

        let expected_storage: SupplyStorage = [
            [b'D', b'A', b'F', b'B'].to_vec(),
            [b'E'].to_vec(),
            [].to_vec(),
            [b'G'].to_vec(),
            [b'H', b'C'].to_vec(),
        ]
        .to_vec();

        move_crates_all_at_once(&mut storage, &transfer);
        assert_eq!(storage, expected_storage);
    }
}
