struct File {
    size: u32,
    name: String,
}

struct Directory {
    parent: Box<Option<Directory>>,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    pub fn from_terminal_output(output: &str) -> Self {
        let base = Self {
            files: vec![],
            directories: vec![],
            parent: Box::new(None),
        };

        let mut current_directory = &base;
        let mut index = 0;

        let lines: Vec<&str> = output.lines().collect();

        for i in 0..lines.len() {
            let line = *lines.get(i).unwrap();

            match line {
                "$ cd /" => {
                    current_directory = &base;
                }
                "$ ls" => {}
                n if n.starts_with('$') => {}
                n => {}
            }
        }

        base
    }
}
