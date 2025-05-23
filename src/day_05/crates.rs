use std::collections::HashMap;

pub type CrateStack = Vec<char>;

pub struct CrateArrangement {
    columns: HashMap<usize, CrateStack>,
}

impl CrateArrangement {
    pub fn from_string(input: &str) -> Self {
        let mut columns: HashMap<usize, CrateStack> = HashMap::new();

        for line in input.lines() {
            let boxes = line
                .replace(&['[', ']'][..], "")
                .replace("   ", "")
                .replace(" ", "_");

            println!("boxes: {}", boxes);

            for (index, next) in boxes.chars().enumerate() {
                if next == '_' {
                    continue;
                }

                println!("box {} in stack {}", next, index);

                let existing = columns.get(&index);

                let mut stack: Vec<char> = vec![];
                match existing {
                    None => {}
                    Some(existing) => stack.extend_from_slice(existing),
                };

                stack.push(next);

                columns.insert(index, stack);
            }

            //
        }

        // let last_line = input.lines().rev().nth(0).unwrap();
        // let width = last_line.len();

        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        Self { columns }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_arrangement() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]";

        let arrangement = CrateArrangement::from_string(input);

        assert_eq!(arrangement.columns.get(&0).unwrap(), &vec!['N', 'Z']);
        assert_eq!(arrangement.columns.get(&1).unwrap(), &vec!['D', 'C', 'M']);
        assert_eq!(arrangement.columns.get(&2).unwrap(), &vec!['P']);
    }
}
