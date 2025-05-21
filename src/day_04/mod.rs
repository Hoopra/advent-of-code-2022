use crate::util::read_input;

#[derive(Debug)]
struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    fn fully_overlaps(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn partially_overlaps(&self, other: &Range) -> bool {
        let other_range = other.lower..=other.upper;

        for i in self.lower..=self.upper {
            if other_range.contains(&i) {
                return true;
            }
        }

        false
    }
}

fn decompose_range(input: &str) -> Range {
    let result: Vec<&str> = input.split('-').collect();

    Range {
        lower: result.get(0).unwrap().parse().unwrap(),
        upper: result.get(1).unwrap().parse().unwrap(),
    }
}

fn find_ranges(input: &str) -> (Range, Range) {
    let result: Vec<&str> = input.split(',').collect();

    (
        decompose_range(result.get(0).unwrap()),
        decompose_range(result.get(1).unwrap()),
    )
}

fn is_full_overlap(r1: &Range, r2: &Range) -> bool {
    r1.fully_overlaps(r2) || r2.fully_overlaps(r1)
}

fn is_partial_overlap(r1: &Range, r2: &Range) -> bool {
    r1.partially_overlaps(r2) || r2.partially_overlaps(r1)
}

fn ranges_fully_overlap(input: &str) -> Option<()> {
    let (r1, r2) = find_ranges(input);

    match is_full_overlap(&r1, &r2) {
        true => Some(()),
        false => None,
    }
}

fn ranges_partially_overlap(input: &str) -> Option<()> {
    let (r1, r2) = find_ranges(input);

    match is_partial_overlap(&r1, &r2) {
        true => Some(()),
        false => None,
    }
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_04/input.txt");

    input
        .lines()
        .filter_map(|line| ranges_fully_overlap(line))
        .count()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_04/input.txt");

    input
        .lines()
        .filter_map(|line| ranges_partially_overlap(line))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn determines_full_overlap() {
        let result = ranges_fully_overlap("6-6,4-6");
        assert_eq!(result, Some(()));

        let result = ranges_fully_overlap("5-7,7-9");
        assert_eq!(result, None);
    }

    #[test]
    fn determines_partial_overlap() {
        let result = ranges_partially_overlap("6-6,4-6");
        assert_eq!(result, Some(()));

        let result = ranges_partially_overlap("5-7,7-9");
        assert_eq!(result, Some(()));

        let result = ranges_partially_overlap("5-6,7-9");
        assert_eq!(result, None);
    }
}
