use crate::util::{read_input, split_string};

fn find_group_badge(input: &str) -> char {
    let mut lines = input.lines();
    let first_line = lines.nth(0).unwrap();
    let second_line = lines.nth(0).unwrap();
    let third_line = lines.nth(0).unwrap();

    first_line
        .chars()
        .find(|c| second_line.contains(*c) && third_line.contains(*c))
        .unwrap()
}

fn find_compartment_duplicates(input: &str) -> Vec<char> {
    let (compartment_1, compartment_2) = split_string(input);

    let mut result: Vec<char> = String::from(compartment_1)
        .chars()
        .filter(|c| compartment_2.contains(*c))
        .collect();

    result.dedup();

    result
}

fn item_priority(c: &char) -> usize {
    let p = c.to_digit(36).unwrap() as usize;

    if c.is_uppercase() {
        return p + 17;
    }

    p - 9
}

fn duplicate_item_priority(input: &str) -> usize {
    let duplicated = find_compartment_duplicates(input);

    duplicated.iter().map(|c| item_priority(c)).sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_03/input.txt");

    input
        .lines()
        .map(|line| duplicate_item_priority(line))
        .sum()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_03/input.txt");
    let mut lines = input.lines();
    let mut sum = 0;

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let group = format!("{}\n{}\n{}", a, b, c);

        let badge = find_group_badge(&group);
        sum += item_priority(&badge);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_duplicate_item_priority() {
        let result = duplicate_item_priority("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(result, 16); // p = 16

        let result = duplicate_item_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(result, 38); // L = 38

        let result = duplicate_item_priority("PmmdzqPrVvPwwTWBwg");
        assert_eq!(result, 42); // P = 42
    }
}
