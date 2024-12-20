use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Elf {
    pub total: u32,
}

impl Elf {
    pub fn from_string(input: &str) -> Self {
        let total = input
            .lines()
            .map(|number| number.parse::<u32>().unwrap())
            .sum();

        Self { total }
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total.cmp(&other.total))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
