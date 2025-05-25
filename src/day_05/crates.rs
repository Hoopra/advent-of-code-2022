use std::{collections::HashMap, vec};

pub type CrateStack = Vec<char>;

#[derive(Debug)]
pub struct CrateArrangement {
    columns: HashMap<usize, CrateStack>,
}

impl CrateArrangement {
    pub fn from_string(input: &str) -> Self {
        let mut indices: Vec<(usize, usize)> = vec![];
        let mut columns: HashMap<usize, CrateStack> = HashMap::new();

        let index_line = input.lines().last().unwrap();

        for (index, character) in index_line.chars().enumerate() {
            if !character.is_numeric() {
                continue;
            }

            let column_number: usize = character.to_string().parse().unwrap();

            indices.push((index, column_number));
            columns.insert(column_number, vec![]);
        }

        for line in input.lines().rev().skip(1) {
            let line_symbols: Vec<char> = line.chars().collect();

            for (index, column_number) in indices.iter() {
                let next = line_symbols.get(*index).unwrap();

                if !next.is_alphabetic() {
                    continue;
                }

                let column = columns.get_mut(column_number).unwrap();
                column.push(*next);
            }
        }

        Self { columns }
    }
}

impl CrateArrangement {
    pub fn move_boxes(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            self.move_box(from, to);
        }
    }

    pub fn move_multiple_boxes(&mut self, from: usize, to: usize, count: usize) {
        let start = self.columns.get(&from).unwrap().clone();
        let mut end = self.columns.get(&to).unwrap().clone();

        let start_boxes = start.len();
        let moved_boxes: Vec<&char> = start.iter().skip(start_boxes - count).collect();

        for moved in moved_boxes.iter() {
            end.push(**moved);
        }

        let new_start = start.into_iter().take(start_boxes - count).collect();

        self.columns.insert(from, new_start);
        self.columns.insert(to, end);
    }

    fn move_box(&mut self, from: usize, to: usize) {
        let mut start = self.columns.get(&from).unwrap().clone();
        let mut end = self.columns.get(&to).unwrap().clone();

        let next = start.pop();

        match next {
            None => {}
            Some(next) => {
                end.push(next);
                self.columns.insert(from, start);
                self.columns.insert(to, end);
            }
        }
    }

    pub fn top_boxes(&self) -> String {
        let mut result = String::new();

        let mut i = 1;
        while let Some(column) = self.columns.get(&i) {
            match column.last() {
                None => {}
                Some(next) => {
                    result.push(*next);
                }
            }
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_arrangement() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        let arrangement = CrateArrangement::from_string(input);

        assert_eq!(arrangement.columns.get(&1).unwrap(), &vec!['Z', 'N']);
        assert_eq!(arrangement.columns.get(&2).unwrap(), &vec!['M', 'C', 'D']);
        assert_eq!(arrangement.columns.get(&3).unwrap(), &vec!['P']);
    }

    #[test]
    fn moves_boxes() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        let mut arrangement = CrateArrangement::from_string(input);

        arrangement.move_boxes(2, 1, 1);

        assert_eq!(arrangement.columns.get(&1).unwrap(), &vec!['Z', 'N', 'D']);
        assert_eq!(arrangement.columns.get(&2).unwrap(), &vec!['M', 'C']);
        assert_eq!(arrangement.columns.get(&3).unwrap(), &vec!['P']);
        assert_eq!(arrangement.columns.get(&4), None);
    }

    #[test]
    fn moves_boxes_full_procedure() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        let mut arrangement = CrateArrangement::from_string(input);

        arrangement.move_boxes(2, 1, 1);
        arrangement.move_boxes(1, 3, 3);
        arrangement.move_boxes(2, 1, 2);
        arrangement.move_boxes(1, 2, 1);

        assert_eq!(arrangement.columns.get(&1).unwrap(), &vec!['C']);
        assert_eq!(arrangement.columns.get(&2).unwrap(), &vec!['M']);
        assert_eq!(
            arrangement.columns.get(&3).unwrap(),
            &vec!['P', 'D', 'N', 'Z']
        );
    }
}
