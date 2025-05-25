use crate::util::read_input;

fn does_symbol_repeat(line: &str) -> bool {
    let mut current = String::new();

    for symbol in line.chars() {
        if current.contains(symbol) {
            return true;
        }

        current.push(symbol);
    }

    false
}

fn find_packet_marker(input: &str, marker_size: usize) -> Option<usize> {
    let mut current_stream = String::new();

    for (index, symbol) in input.chars().enumerate() {
        current_stream.push(symbol);
        if current_stream.len() > marker_size {
            current_stream = current_stream.chars().skip(1).collect();
        }

        if current_stream.len() >= marker_size && !does_symbol_repeat(&current_stream) {
            return Some(index + 1);
        }
    }

    None
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_06/input.txt");

    find_packet_marker(&input, 4).unwrap()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_06/input.txt");

    find_packet_marker(&input, 14).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_packet_marker() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = find_packet_marker(input, 4);

        assert_eq!(result, Some(7));
    }
}
