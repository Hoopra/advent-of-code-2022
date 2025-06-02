mod map_2d;

use map_2d::Map2d;

use crate::util::read_input;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_08/input.txt");

    let map = Map2d::from_string(&input);

    map.count_visible_trees()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_08/input.txt");

    let map = Map2d::from_string(&input);

    map.best_scenic_score()
}
