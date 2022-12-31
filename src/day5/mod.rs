use std::fs;

pub fn main() {
    let contents =
        fs::read_to_string("./src/day5/input.txt").expect("Should have been able to read the file");

    let blocks: Vec<_> = contents.split("\n\n").collect();

    let mut first_block: Vec<_> = blocks.get(0).unwrap().split('\n').collect();

    let mut crates: Vec<Vec<char>> = vec![];

    let indexes_str = first_block.pop().unwrap();

    first_block.reverse();

    indexes_str
        .split("")
        .enumerate()
        .for_each(|(i, index_str)| {
            let index_res = index_str.parse::<u32>();

            if index_res.is_ok() {
                let mut row: Vec<char> = vec![];

                first_block.iter().for_each(|&line| {
                    let c = line.chars().nth(i - 1).unwrap();

                    if c != ' ' {
                        row.push(c);
                    }
                });

                crates.push(row);
            }
        });

    let mut crates_part1 = crates.clone();

    let mut crates_part2 = crates.clone();

    let second_block: Vec<_> = blocks.get(1).unwrap().split('\n').collect();

    second_block.iter().for_each(|&line| {
        let split_line = line.split(' ').collect::<Vec<_>>();

        fn get_usize_from_index(split_line: &Vec<&str>, index: usize) -> usize {
            split_line.get(index).unwrap().parse::<usize>().unwrap()
        }

        let amount = get_usize_from_index(&split_line, 1);
        let from = get_usize_from_index(&split_line, 3);
        let to = get_usize_from_index(&split_line, 5);

        // Part 1
        (0..amount).into_iter().for_each(|_| {
            let row_from = &mut crates_part1[from - 1];

            let c = &row_from.pop().unwrap();

            let row_to = &mut crates_part1[to - 1];

            row_to.push(c.clone());
        });

        // Part 2
        {
            let row_from = &mut crates_part2[from - 1];

            let cut: Vec<_> = row_from.splice(row_from.len() - amount.., []).collect();

            let row_to = &mut crates_part2[to - 1];

            row_to.splice(row_to.len().., cut);
        }
    });

    fn get_top_of_each_row(crates: Vec<Vec<char>>) -> String {
        crates
            .iter()
            .map(|row| row.last().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    println!("> Day 5");

    println!("Part 1: Answer is {}", get_top_of_each_row(crates_part1));

    println!("Part 2: Answer is {}", get_top_of_each_row(crates_part2));
}
