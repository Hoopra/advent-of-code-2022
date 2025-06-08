use std::collections::HashMap;

#[derive(Debug)]
struct File {
    size: u32,
    #[allow(unused)]
    name: String,
}

impl File {
    pub fn new(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    parent_name: Option<String>,
    files: Vec<File>,
    children: Vec<String>,
}

impl Directory {
    pub fn total_size(&self, directories: &HashMap<String, Directory>) -> u32 {
        let my_size = self.total_file_size();
        let mut total_child_size = 0;

        for child_name in &self.children {
            let child = directories.get(child_name).unwrap();
            total_child_size += child.total_size(directories);
        }

        my_size + total_child_size
    }

    pub fn total_file_size(&self) -> u32 {
        self.files.iter().fold(0, |result, file| result + file.size)
    }
}

pub fn parse_terminal_output(output: &str) -> HashMap<String, Directory> {
    let base = Directory {
        name: "/".to_string(),
        files: vec![],
        children: vec![],
        parent_name: None,
    };

    let mut current_directory = "/".to_string();

    let mut directories = HashMap::new();
    directories.insert(current_directory.clone(), base);

    let lines: Vec<&str> = output.lines().collect();

    for i in 0..lines.len() {
        let line = *lines.get(i).unwrap();

        match line {
            "$ cd /" => {
                current_directory = "/".to_string();
            }
            "$ cd .." => {
                let directory = directories.get(&current_directory).unwrap();
                current_directory = directory.parent_name.as_ref().unwrap().to_string();
            }
            n if n.starts_with("$ cd ") => {
                let name: String = n.chars().skip(5).collect();
                current_directory = name;
            }
            "$ ls" => {
                let contents: Vec<&&str> = lines
                    .iter()
                    .skip(i + 1)
                    .take_while(|line| !line.starts_with('$'))
                    .collect();

                let mut files = vec![];
                let mut children = vec![];

                for entry in contents {
                    if entry.starts_with("dir") {
                        let name: String = entry.chars().skip(4).collect();

                        let directory = Directory {
                            name: name.clone(),
                            parent_name: Some(current_directory.clone()),
                            files: vec![],
                            children: vec![],
                        };

                        children.push(name.clone());
                        directories.insert(name, directory);
                    } else {
                        let mut components = entry.split(" ");
                        let size = components.next().unwrap().parse().unwrap();
                        let name = components.next().unwrap();

                        files.push(File::new(name, size));
                    }
                }

                directories.get_mut(&current_directory).unwrap().files = files;
                directories.get_mut(&current_directory).unwrap().children = children;
            }
            n if n.starts_with('$') => {}
            _ => {}
        }
    }

    directories
}

fn indented_string(indentation: u16) -> String {
    (0..=indentation).map(|_| " ").collect()
}

pub fn print_directories(
    directory: &Directory,
    structure: &HashMap<String, Directory>,
    indentation: u16,
) {
    println!("{}- {} (dir)", indented_string(indentation), directory.name);
    for file in &directory.files {
        println!(
            "{}- {} (file, size={})",
            indented_string(indentation + 2),
            file.name,
            file.size
        );
    }

    for subdir in &directory.children {
        let directory = structure.get(subdir).unwrap();
        print_directories(directory, structure, indentation + 2);
    }
}

pub fn list_directories_by_size(directories: &HashMap<String, Directory>) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    directories.values().into_iter().for_each(|directory| {
        let size = directory.total_size(directories);

        println!("name {}, size {}", directory.name, size);
        result.insert(directory.name.clone(), size);
    });

    result
}

pub fn sum_directories_with_max_size(
    directories: &HashMap<String, Directory>,
    max_size: u32,
) -> u32 {
    let by_size = list_directories_by_size(directories);

    by_size
        .values()
        .fold(0, |result, size| match size <= &max_size {
            true => result + size,
            false => result,
        })
}
