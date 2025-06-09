mod monkey;

use std::collections::HashMap;

use crate::{day_11::monkey::Monkey, util::read_input};

fn simulate_monkey_rounds_from_input(input: &str, rounds: u32) -> u32 {
    let mut monkeys: HashMap<usize, Monkey> = input
        .split("\n\n")
        .enumerate()
        .map(|(index, text)| (index, Monkey::from_text(text)))
        .collect();

    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&index).unwrap();

            let passed = monkey.inspect_items();

            for (i, passed_item) in passed {
                let monkey = monkeys.get_mut(&i).unwrap();
                monkey.add_item(passed_item);
            }
        }

        monkeys.iter().for_each(|(num, monkey)| {
            println!(
                "monkey {} inspections: {}, items: {}",
                num,
                monkey.total_inspections,
                monkey.count_items()
            );
        });
    }

    monkeys
        .values()
        .fold(1, |result, monkey| result * monkey.total_inspections)
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_11/input.txt");

    simulate_monkey_rounds_from_input(&input, 20)
}

pub fn solve_part_2() -> i32 {
    let input = read_input("src/day_11/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulates_1_monkey_round_from_input() {
        let input = "Monkey 0:\nStarting items: 79, 98\nOperation: new = old * 19\nTest: divisible by 23\nIf true: throw to monkey 2\nIf false: throw to monkey 3\n\nMonkey 1:\nStarting items: 54, 65, 75, 74\nOperation: new = old + 6\nTest: divisible by 19\nIf true: throw to monkey 2\nIf false: throw to monkey 0\n\nMonkey 2:\nStarting items: 79, 60, 97\nOperation: new = old * old\nTest: divisible by 13\nIf true: throw to monkey 1\nIf false: throw to monkey 3\n\nMonkey 3:\nStarting items: 74\nOperation: new = old + 3\nTest: divisible by 17\nIf true: throw to monkey 0\nIf false: throw to monkey 1";

        let result = simulate_monkey_rounds_from_input(input, 1);
        assert_eq!(result, 10605);
    }
}
