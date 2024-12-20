mod elf;

use crate::util::read_input;
use elf::Elf;

fn find_max_elf_calories(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|input| Elf::from_string(input).total)
        .max()
        .unwrap()
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_01/input.txt");

    find_max_elf_calories(&input)
}

fn find_most_elf_calories(input: &str, take: usize) -> u32 {
    let mut elves: Vec<Elf> = input
        .split("\n\n")
        .map(|input| Elf::from_string(input))
        .collect();

    elves.sort();

    elves.iter().rev().take(take).map(|elf| elf.total).sum()
}

pub fn solve_part_2() -> u32 {
    let input = read_input("src/day_01/input.txt");

    find_most_elf_calories(&input, 3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_max_elf_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        let result = find_max_elf_calories(input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn finds_most_elf_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        let result = find_most_elf_calories(input, 3);
        assert_eq!(result, 45000);
    }
}
