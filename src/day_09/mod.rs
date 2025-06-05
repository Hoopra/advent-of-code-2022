mod rope;

use crate::util::read_input;
use rope::determine_rope_tail_steps;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_09/input.txt");

    determine_rope_tail_steps(&input, 2).len()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_09/input.txt");

    determine_rope_tail_steps(&input, 10).len()
}
