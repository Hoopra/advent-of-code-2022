mod game;

use crate::util::read_input;
use game::Game;

fn find_win_score(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_string(line).score())
        .sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_02/input.txt");

    find_win_score(&input)
}

fn find_win_score_given_result(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_string_with_result(line).score())
        .sum()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_02/input.txt");

    find_win_score_given_result(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_win_score() {
        let input = "A Y\nB X\nC Z";

        let result = find_win_score(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn finds_win_score_given_result() {
        let input = "A Y\nB X\nC Z";

        let result = find_win_score_given_result(input);
        assert_eq!(result, 12);
    }
}
