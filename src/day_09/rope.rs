use crate::model::{Direction, Position2d};
use std::collections::HashSet;

type Position = Position2d<isize>;

struct Rope {
    head_position: Position,
    tail_positions: Vec<Position>,

    tail_visits: HashSet<Position>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        let mut tail_positions = vec![];

        for _ in 0..(knots - 1) {
            tail_positions.push((0, 0));
        }

        Self {
            head_position: (0, 0),
            tail_positions,
            tail_visits: HashSet::new(),
        }
    }
}

fn calculate_position_difference(x_head: &isize, x_tail: &isize) -> (isize, isize) {
    let difference_x = x_head - x_tail;
    let difference_x_abs = difference_x.abs();
    let direction_x = match difference_x_abs {
        0 => 0,
        _ => difference_x / difference_x_abs,
    };

    (difference_x_abs, direction_x)
}

fn minimise_distance_step(difference_abs: isize, direction: isize, is_diagonal: bool) -> isize {
    match difference_abs {
        2 => 1 * direction,
        1 if is_diagonal => 1 * direction,
        _ => 0,
    }
}

fn minimise_distance_between(head: &Position, tail: &Position) -> Position {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;

    let (diff_x_abs, direction_x) = calculate_position_difference(x_head, x_tail);
    let (diff_y_abs, direction_y) = calculate_position_difference(y_head, y_tail);

    let is_diagonal = diff_x_abs + diff_y_abs > 2;

    let new_x_tail = x_tail + minimise_distance_step(diff_x_abs, direction_x, is_diagonal);
    let new_y_tail = y_tail + minimise_distance_step(diff_y_abs, direction_y, is_diagonal);

    (new_x_tail, new_y_tail)
}

impl Rope {
    pub fn move_steps(&mut self, steps: &Vec<(Direction, isize)>) {
        for (direction, count) in steps {
            self.move_head(direction, *count);
        }
    }

    fn move_head(&mut self, direction: &Direction, steps: isize) {
        for _ in 0..steps {
            self.move_head_once(&direction);
            self.move_tail_once();
        }
    }

    fn move_head_once(&mut self, direction: &Direction) {
        let (x, y) = self.head_position;

        match direction {
            Direction::Up => self.head_position = (x, y + 1),
            Direction::Down => self.head_position = (x, y - 1),
            Direction::Left => self.head_position = (x - 1, y),
            Direction::Right => self.head_position = (x + 1, y),
        }
    }

    fn move_tail_once(&mut self) {
        let mut new_tail = vec![];

        for tail_position in &self.tail_positions {
            let next = new_tail.last().unwrap_or(&self.head_position);

            let new_tail_position = minimise_distance_between(next, &tail_position);
            new_tail.push(new_tail_position);
        }

        self.tail_positions = new_tail;

        let visited = self.tail_positions.last().unwrap();
        self.tail_visits.insert(*visited);
    }
}

pub fn parse_steps(input: &str) -> Vec<(Direction, isize)> {
    input
        .lines()
        .map(|line| {
            let mut components = line.split(" ");

            let direction = match components.nth(0).unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                _ => Direction::Down,
            };

            let steps = components.nth(0).unwrap().parse().unwrap();

            (direction, steps)
        })
        .collect()
}

pub fn determine_rope_tail_steps(input: &str, knots: usize) -> HashSet<Position> {
    let mut rope = Rope::new(knots);
    let steps = parse_steps(input);

    rope.move_steps(&steps);

    rope.tail_visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_rope_1_knot() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let steps = determine_rope_tail_steps(input, 2);

        assert_eq!(steps.len(), 13);
    }

    #[test]
    fn moves_rope_10_knots() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let steps = determine_rope_tail_steps(input, 10);

        assert_eq!(steps.len(), 36);
    }
}
