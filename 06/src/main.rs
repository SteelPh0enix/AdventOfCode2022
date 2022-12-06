use std::{collections::HashSet, fs};

fn find_start_marker_index(data: &str, length: usize) -> usize {
    for (index, window) in data.as_bytes().windows(length).enumerate() {
        // could use `.unique()` from itertools here to make it one-liner, but w/e
        let set: HashSet<u8> = HashSet::from_iter(window.iter().cloned());
        if set.len() == window.len() {
            return index + length;
        }
    }
    0usize
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let packet_marker_index = find_start_marker_index(input.as_str(), 4);
    println!("Packet start marker is @ {packet_marker_index}");
    let message_marker_index = find_start_marker_index(input.as_str(), 14);
    println!("Message start marker is @ {message_marker_index}");
}

#[cfg(test)]
mod tests {
    use crate::find_start_marker_index;

    #[test]
    fn finds_start_marker_index_correctly() {
        let input = "qweqaweqddss";
        let expected_marker_index = 5usize;
        assert_eq!(find_start_marker_index(input, 4), expected_marker_index);
    }
}
