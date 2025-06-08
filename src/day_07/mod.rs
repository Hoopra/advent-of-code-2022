mod directory;

use directory::{parse_terminal_output, print_directories, sum_directories_with_max_size};

use crate::util::read_input;

fn find_directories_with_max_size_by_output(output: &str, max_size: u32) -> u32 {
    let directories = parse_terminal_output(output);

    let base = directories.get("/").unwrap();
    print_directories(&base, &directories, 0);

    // for (_, directory) in &directories {
    //     println!("{:?}", directory);
    // }

    sum_directories_with_max_size(&directories, max_size)
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_07/input.txt");

    find_directories_with_max_size_by_output(&input, 100000)
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_07/input.txt");

    0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_directories_with_max_size_by_output() {
        let output = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let result = find_directories_with_max_size_by_output(output, 100000);

        assert_eq!(result, 95437);
        assert_eq!(true, false)
    }
}
