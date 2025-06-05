use crate::model::Position2d;
use std::{collections::HashMap, ops::Range};

type Position = Position2d<usize>;

pub struct Map2d {
    tiles: HashMap<Position, usize>,
    size_x: usize,
    size_y: usize,
}

impl Map2d {
    pub fn from_string(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut size_x = 0;
        let mut size_y = 0;

        input.lines().enumerate().for_each(|(y, line)| {
            size_y = y;

            line.chars().enumerate().for_each(|(x, height)| {
                let height = height.to_string().parse().unwrap();

                tiles.insert((x, y), height);
                size_x = x;
            });
        });

        Self {
            tiles,
            size_x,
            size_y,
        }
    }
}

fn to_x_positions(range: impl Iterator<Item = usize>, y: usize) -> Vec<Position> {
    range.map(|x| (x, y)).collect()
}

fn to_y_positions(range: impl Iterator<Item = usize>, x: usize) -> Vec<Position> {
    range.map(|y| (x, y)).collect()
}

impl Map2d {
    fn find_visible_trees(&self) -> HashMap<Position, bool> {
        let mut visibility = HashMap::new();

        for y in 0..=self.size_y {
            self.traverse_column(y, 0..(self.size_x + 1), &mut visibility);
        }

        for x in 0..=self.size_x {
            self.traverse_row(x, 0..(self.size_y + 1), &mut visibility);
        }

        visibility
    }

    pub fn count_visible_trees(&self) -> usize {
        self.find_visible_trees()
            .values()
            .fold(0, |total, is_visible| match is_visible {
                true => total + 1,
                false => total,
            })
    }

    fn traverse_column(
        &self,
        y: usize,
        rows: Range<usize>,
        visibility: &mut HashMap<Position, bool>,
    ) {
        let mut max_height = 0;

        for x in rows.clone() {
            max_height = self.determine_if_visible(x, y, max_height, visibility);
        }

        max_height = 0;

        for x in rows.rev() {
            max_height = self.determine_if_visible(x, y, max_height, visibility);
        }
    }

    fn traverse_row(
        &self,
        x: usize,
        columns: Range<usize>,
        visibility: &mut HashMap<Position, bool>,
    ) {
        let mut max_height = 0;

        for y in columns.clone() {
            max_height = self.determine_if_visible(x, y, max_height, visibility);
        }

        max_height = 0;

        for y in columns.rev() {
            max_height = self.determine_if_visible(x, y, max_height, visibility);
        }
    }

    fn determine_if_visible(
        &self,
        x: usize,
        y: usize,
        max_height: usize,
        visibility: &mut HashMap<Position, bool>,
    ) -> usize {
        let position = (x, y);

        let visible = visibility.get(&position);

        let height = self.tiles.get(&position).unwrap();
        let is_edge = y == 0 || x == 0 || y == self.size_y || x == self.size_x;
        let new_max_height = match height > &max_height {
            true => *height,
            false => max_height,
        };

        match visible {
            Some(visible) if *visible => {
                return new_max_height;
            }
            _ => {}
        }

        let visible = match is_edge {
            true => true,
            false => height > &max_height,
        };

        visibility.insert(position, visible);

        new_max_height
    }

    fn find_scenic_scores(&self) -> HashMap<Position, usize> {
        let mut scores = HashMap::new();

        for x in 0..=self.size_x {
            for y in 0..=self.size_y {
                let score = self.scenic_score_at(x, y);

                scores.insert((x, y), score);
            }
        }

        scores
    }

    fn scenic_score_at(&self, x: usize, y: usize) -> usize {
        let position = (x, y);

        let vantage = self.tiles.get(&position).unwrap();

        let ranges = vec![
            to_x_positions((0..x).into_iter().rev(), y),
            to_x_positions((x + 1)..(self.size_x + 1), y),
            to_y_positions((0..y).into_iter().rev(), x),
            to_y_positions((y + 1)..(self.size_y + 1), x),
        ];

        ranges
            .into_iter()
            .map(|range| {
                let mut range_score = 0;

                for position in range {
                    let height = self.tiles.get(&position).unwrap();

                    range_score += 1;

                    if height >= vantage {
                        break;
                    }
                }

                range_score
            })
            .fold(1, |result, direction_score| result * direction_score)
    }

    pub fn best_scenic_score(&self) -> usize {
        *self.find_scenic_scores().values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_visible_trees() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let map = Map2d::from_string(input);

        let result = map.count_visible_trees();
        assert_eq!(result, 21)
    }
    #[test]
    fn counts_visible_trees_small_example() {
        let input = "1111\n1201\n1021\n1111";
        let map = Map2d::from_string(input);

        let visibility = map.find_visible_trees();
        assert_eq!(visibility.get(&(1, 1)), Some(&true));
        assert_eq!(visibility.get(&(2, 1)), Some(&false));
        assert_eq!(visibility.get(&(1, 2)), Some(&false));
        assert_eq!(visibility.get(&(2, 2)), Some(&true));

        let result = map.count_visible_trees();
        assert_eq!(result, 14);
    }

    #[test]
    fn determines_visible_trees() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let map = Map2d::from_string(input);
        let visibility = map.find_visible_trees();

        assert_eq!(visibility.get(&(1, 1)), Some(&true));
        assert_eq!(visibility.get(&(2, 1)), Some(&true));
        assert_eq!(visibility.get(&(0, 2)), Some(&true));
        assert_eq!(visibility.get(&(3, 1)), Some(&false));
    }

    #[test]
    fn determines_scenic_score_at_position() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let map = Map2d::from_string(input);

        assert_eq!(map.scenic_score_at(2, 1), 4);
        assert_eq!(map.scenic_score_at(2, 3), 8);
    }

    #[test]
    fn determines_best_scenic_score() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let map = Map2d::from_string(input);
        let best = map.best_scenic_score();

        assert_eq!(best, 8);
    }
}
