use std::fs;

use crate::utils::get_total;

pub fn main() {
    let contents =
        fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents.split("\n").collect();

    let mut alphabet_lowercase: Vec<_> = ('a'..='z').into_iter().collect();
    let mut alphabet_uppercase: Vec<_> = ('A'..='Z').into_iter().collect();

    alphabet_lowercase.append(&mut alphabet_uppercase);

    let combined_alphabets = alphabet_lowercase;

    let values = lines.iter().map(|&line| {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        let mut common_char: Option<char> = None;

        let first_chars: Vec<_> = first_half.chars().collect();
        let second_chars: Vec<_> = second_half.chars().collect();

        let mut i = 0;

        while common_char.is_none() && i < first_chars.len() {
            let char_from_first = first_chars.get(i).unwrap().clone();

            i += 1;

            let mut j = 0;

            while common_char.is_none() && j < second_chars.len() {
                let char_from_second = second_chars.get(j).unwrap().clone();

                if char_from_first == char_from_second {
                    common_char = Some(char_from_first);
                }

                j += 1;
            }
        }

        (combined_alphabets
            .iter()
            .position(|&c| c == common_char.unwrap())
            .unwrap()
            + 1) as i32
    });

    let total_part1 = get_total(values);

    let values = lines.chunks(3).map(|chunk| {
        let line1: Vec<_> = chunk[0].chars().collect();
        let line2: Vec<_> = chunk[1].chars().collect();
        let line3: Vec<_> = chunk[2].chars().collect();

        let mut common_char: Option<char> = None;

        let mut i = 0;

        while common_char.is_none() && i < line1.len() {
            let char_from_first = line1.get(i).unwrap().clone();

            i += 1;

            let mut j = 0;

            while common_char.is_none() && j < line2.len() {
                let char_from_second = line2.get(j).unwrap().clone();

                j += 1;

                let mut k = 0;

                while common_char.is_none() && k < line3.len() {
                    let char_from_third = line3.get(k).unwrap().clone();

                    k += 1;

                    if char_from_first == char_from_second && char_from_second == char_from_third {
                        common_char = Some(char_from_first);
                    }
                }
            }
        }

        (combined_alphabets
            .iter()
            .position(|&c| c == common_char.unwrap())
            .unwrap()
            + 1) as i32
    });

    let total_part2 = get_total(values);

    println!("> Day 3");

    println!("Part 1: Answer is {total_part1}");
    println!("Part 2: Answer is {total_part2}");
}
