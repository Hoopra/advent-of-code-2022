use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value {
    Old,
    Constant(i32),
}

impl Value {
    pub fn from_text(input: &str) -> Self {
        match input {
            "old" => Self::Old,
            n => Self::Constant(n.parse().unwrap()),
        }
    }
}

impl Value {
    fn extract(&self, item_level: i32) -> i32 {
        match self {
            Value::Old => item_level,
            Value::Constant(c) => *c,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MonkeyOperation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl MonkeyOperation {
    pub fn from_text(input: &str) -> Self {
        let parameters = input.split("Operation: new = ").nth(1).unwrap();
        let mut symbols = parameters.split(" ");
        let first = Value::from_text(symbols.next().unwrap());
        let operator = symbols.next().unwrap();
        let second = Value::from_text(symbols.next().unwrap());

        match operator {
            "+" => MonkeyOperation::Add(first, second),
            "*" | _ => MonkeyOperation::Multiply(first, second),
        }
        // Operation: new = old + 2
    }
}

#[derive(Debug, PartialEq)]
pub struct MonkeyCondition {
    divisible_test: i32,
    false_recipient: usize,
    true_recipient: usize,
}

impl MonkeyCondition {
    fn from_text(test: &str, if_true: &str, if_false: &str) -> Self {
        MonkeyCondition {
            divisible_test: value_after(test, "divisible by "),
            true_recipient: value_after(if_true, "throw to monkey ") as usize,
            false_recipient: value_after(if_false, "throw to monkey ") as usize,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i32>,
    operation: MonkeyOperation,
    test: MonkeyCondition,
    pub total_inspections: u32,
}

impl Monkey {
    pub fn from_text(input: &str) -> Self {
        let mut lines = input.split("\n");
        let _name = lines.next().unwrap();
        let starting_items = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let test = lines.next().unwrap();
        let if_true = lines.next().unwrap();
        let if_false = lines.next().unwrap();

        Self {
            items: parse_items(starting_items),
            test: MonkeyCondition::from_text(test, if_true, if_false),
            operation: MonkeyOperation::from_text(operation),
            total_inspections: 0,
        }
    }
}

impl Monkey {
    fn inspect_item(&mut self) -> (usize, i32) {
        let item = self.items.pop().unwrap();

        let mut level = match &self.operation {
            MonkeyOperation::Add(a, b) => {
                let a = a.extract(item);
                let b = b.extract(item);

                a + b
            }
            MonkeyOperation::Multiply(a, b) => {
                let a = a.extract(item);
                let b = b.extract(item);

                a * b
            }
        };

        level = level / 3;
        self.total_inspections += 1;

        let MonkeyCondition {
            divisible_test,
            false_recipient,
            true_recipient,
        } = self.test;

        match level % divisible_test == 0 {
            true => (true_recipient, level),
            false => (false_recipient, level),
        }
    }

    pub fn inspect_items(&mut self) -> HashMap<usize, i32> {
        let mut result = HashMap::new();

        for _ in 0..self.items.len() {
            let passed = self.inspect_item();
            result.insert(passed.0, passed.1);
        }

        result
    }

    pub fn add_item(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}

fn parse_items(input: &str) -> Vec<i32> {
    let items = input.split("items: ").nth(1).unwrap();
    items
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn value_after(input: &str, pattern: &str) -> i32 {
    input.split(pattern).nth(1).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_monkey_from_text() {
        let input = "Monkey 1:\nStarting items: 71, 55, 82\nOperation: new = old + 2\nTest: divisible by 13\nIf true: throw to monkey 7\nIf false: throw to monkey 2";
        let monkey = Monkey::from_text(input);

        assert_eq!(monkey.items, vec![71, 55, 82]);
        assert_eq!(
            monkey.operation,
            MonkeyOperation::Add(Value::Old, Value::Constant(2))
        );
        assert_eq!(
            monkey.test,
            MonkeyCondition {
                divisible_test: 13,
                false_recipient: 2,
                true_recipient: 7
            }
        );
    }
}
