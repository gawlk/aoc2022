use std::fs;

fn get_numbers_vec_from_split_string(split: &Vec<&str>, index: usize) -> Vec<u32> {
    split
        .get(index)
        .unwrap()
        .split('-')
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

pub fn main() {
    let contents =
        fs::read_to_string("./src/day4/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents.split("\n").collect();

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    lines.iter().for_each(|&line| {
        let split: Vec<_> = line.split(',').into_iter().collect();

        let part1_split: Vec<_> = get_numbers_vec_from_split_string(&split, 0);
        let &part1_from = part1_split.get(0).unwrap();
        let &part1_to = part1_split.get(1).unwrap();
        let part1_range = part1_from..=part1_to;

        let part2_split: Vec<_> = get_numbers_vec_from_split_string(&split, 1);
        let &part2_from = part2_split.get(0).unwrap();
        let &part2_to = part2_split.get(1).unwrap();

        if (part1_from <= part2_from && part1_to >= part2_to)
            || (part2_from <= part1_from && part2_to >= part1_to)
        {
            total_part1 += 1;
            total_part2 += 1;
        } else if part1_range.contains(&part2_from) || part1_range.contains(&part2_to) {
            total_part2 += 1;
        }
    });

    println!("> Day 4");

    println!("Part 1: Answer is {total_part1}");

    println!("Part 2: Answer is {total_part2}");
}
