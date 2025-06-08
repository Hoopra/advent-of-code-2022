pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn from_string(input: &str) -> Self {
        match input {
            "noop" => Self::Noop,
            s => {
                let number: i32 = s.split(" ").nth(1).unwrap().parse().unwrap();
                Self::Addx(number)
            }
        }
    }

    pub fn multiple_from_string(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| Instruction::from_string(line))
            .collect()
    }
}

pub struct Program {
    cycle: u32,
    register_value: i32,
}

impl Program {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            register_value: 1,
        }
    }
}

impl Program {
    fn capture_recording(&self, record_at: &u32, current: Option<i32>) -> Option<i32> {
        match current {
            Some(current) => Some(current),
            None => match &self.cycle == record_at {
                false => None,
                true => Some(self.register_value),
            },
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction, record_at: &u32) -> Option<i32> {
        let mut recorded = self.capture_recording(record_at, None);

        match instruction {
            Instruction::Noop => self.cycle += 1,
            Instruction::Addx(value) => {
                self.cycle += 1;
                recorded = self.capture_recording(record_at, recorded);
                self.cycle += 1;
                recorded = self.capture_recording(record_at, recorded);
                self.register_value += value;
            }
        }

        self.capture_recording(record_at, recorded)
    }

    pub fn record_execution(
        &mut self,
        instructions: &Vec<Instruction>,
        record_times: &[u32],
    ) -> Vec<i32> {
        let mut recordings: Vec<i32> = vec![];
        let mut record_times = record_times.iter();
        let mut record_at = record_times.next();

        for instruction in instructions {
            match record_at {
                None => break,
                _ => {}
            }

            let recording = self.execute_instruction(instruction, record_at.unwrap());

            match recording {
                None => {}
                Some(recording) => {
                    recordings.push(recording);
                    record_at = record_times.next();
                }
            }
        }

        recordings
    }

    fn execute_instruction_full_record(&mut self, instruction: &Instruction) -> Vec<i32> {
        let mut result = vec![];

        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
                result.push(self.register_value);
            }
            Instruction::Addx(value) => {
                self.cycle += 1;
                result.push(self.register_value);
                self.cycle += 1;
                result.push(self.register_value);
                self.register_value += value;
            }
        }

        result
    }

    pub fn draw_crt(&mut self, instructions: &Vec<Instruction>) -> String {
        let mut result = String::new();

        let mut cycle = 0;

        for instruction in instructions {
            let results = self.execute_instruction_full_record(instruction);

            for value in results {
                let pointer = cycle % 40;
                if pointer == 0 {
                    result.push('\n');
                }

                println!(
                    "pixel position: {}, sprites: {},{},{}",
                    pointer,
                    value - 1,
                    value,
                    value + 1,
                );

                println!(
                    "test: {} <= {} <= {} ({})",
                    value - 1,
                    pointer,
                    value + 1,
                    value - 1 <= pointer && value + 1 >= pointer
                );

                match value - 1 <= pointer && value + 1 >= pointer {
                    true => result.push('#'),
                    false => result.push('.'),
                }

                cycle += 1;
            }
        }

        result
    }
}
