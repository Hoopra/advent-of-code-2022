mod crates;

use crate::util::read_input;
use crates::CrateArrangement;

#[derive(Debug)]
struct Procedure {
    from: usize,
    to: usize,
    count: usize,
}

fn to_number(input: Option<&str>) -> usize {
    input.unwrap().parse().unwrap()
}

impl Procedure {
    pub fn from_string(input: &str) -> Self {
        let formatted = input
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");

        let mut components = formatted.split(',');

        Self {
            count: to_number(components.next()),
            from: to_number(components.next()),
            to: to_number(components.next()),
        }
    }
}

fn parse_input(input: &str) -> (CrateArrangement, Vec<Procedure>) {
    let mut components = input.split("\n\n");

    let arrangement_text = components.next().unwrap();
    let procedure_text = components.next().unwrap();

    let arrangement = CrateArrangement::from_string(arrangement_text);

    let procedures: Vec<Procedure> = procedure_text
        .lines()
        .map(|l| Procedure::from_string(l))
        .collect();

    (arrangement, procedures)
}

fn determine_top_boxes_after_procedures(input: &str) -> String {
    let (mut arrangement, procedures) = parse_input(&input);

    for procedure in procedures {
        arrangement.move_boxes(procedure.from, procedure.to, procedure.count);
    }

    arrangement.top_boxes()
}

pub fn solve_part_1() -> String {
    let input = read_input("src/day_05/input.txt");

    determine_top_boxes_after_procedures(&input)
}

fn determine_top_boxes_after_procedures_multiple(input: &str) -> String {
    let (mut arrangement, procedures) = parse_input(&input);

    for procedure in procedures {
        arrangement.move_multiple_boxes(procedure.from, procedure.to, procedure.count);
    }

    arrangement.top_boxes()
}

pub fn solve_part_2() -> String {
    let input = read_input("src/day_05/input.txt");

    determine_top_boxes_after_procedures_multiple(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn determines_final_arrangement() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let result = determine_top_boxes_after_procedures(input);
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn determines_final_arrangement_multiple_boxes() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let result = determine_top_boxes_after_procedures_multiple(input);
        assert_eq!(result, "MCD".to_string());
    }
}
