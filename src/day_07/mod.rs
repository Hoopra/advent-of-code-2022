mod directory;

use directory::{
    parse_terminal_output, smallest_deletable_directory_size, sum_directories_with_max_size,
};

use crate::util::read_input;

fn find_directories_with_max_size_by_output(output: &str, max_size: u32) -> u32 {
    let directories = parse_terminal_output(output);

    sum_directories_with_max_size(&directories, max_size)
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_07/input.txt");

    find_directories_with_max_size_by_output(&input, 100000)
}

fn find_smallest_deletable_directory_by_output(output: &str) -> u32 {
    smallest_deletable_directory_size(output, 30000000, 70000000)
}

pub fn solve_part_2() -> u32 {
    let input = read_input("src/day_07/input.txt");

    find_smallest_deletable_directory_by_output(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_directories_with_max_size_by_output() {
        let output = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let result = find_directories_with_max_size_by_output(output, 100000);

        assert_eq!(result, 95437);
    }

    #[test]
    fn finds_smallest_deletable_directory() {
        let output = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let result = find_smallest_deletable_directory_by_output(output);

        assert_eq!(result, 24933642);
    }
}
