use std::fs;

use crate::utils::get_total;

pub fn main() {
    let contents =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");

    let blocks: Vec<_> = contents.split("\n\n").collect();

    let mut total_carried_per_elf: Vec<_> = blocks
        .iter()
        .map(|&block| {
            let lines: Vec<_> = block.split("\n").collect();

            let calories_carried = lines.iter().map(|&line| line.parse::<i32>().unwrap());

            let total_of_calories_carried = get_total(calories_carried);

            return total_of_calories_carried;
        })
        .collect();

    total_carried_per_elf.sort();

    let top_amount_carried = total_carried_per_elf.pop().unwrap();

    println!("> Day 1");

    println!("Part 1: Answer is {top_amount_carried}");

    let second_top_amount_carried = total_carried_per_elf.pop().unwrap();
    let third_top_amount_carried = total_carried_per_elf.pop().unwrap();

    let tops_amount_carried =
        top_amount_carried + second_top_amount_carried + third_top_amount_carried;

    println!("Part 2: Answer is {tops_amount_carried}");
}
