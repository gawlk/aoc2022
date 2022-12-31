use std::{collections::HashSet, fs};

pub fn main() {
    let contents =
        fs::read_to_string("./src/day6/input.txt").expect("Should have been able to read the file");

    let chars: Vec<_> = contents.chars().collect();

    fn find_marker(chars: &Vec<char>, packet_size: usize) -> usize {
        let mut i = 0;

        let size = packet_size;

        let mut range_end = 0;

        while i <= chars.len() - size {
            range_end = i + size;

            let slice = &chars[i..range_end];

            let len = slice.into_iter().collect::<HashSet<_>>().len();

            if len == size {
                break;
            }

            i += 1;
        }

        return range_end;
    }

    println!("> Day 6");

    println!("Part 1: Answer is {}", find_marker(&chars, 4));

    println!("Part 2: Answer is {}", find_marker(&chars, 14));
}
