use std::fs;

mod part1;
mod part2;

pub fn main() {
    let contents =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents.split("\n").collect();

    let mut total_part1 = 0;

    let mut total_part2 = 0;

    lines.iter().for_each(|&line| {
        let opponent_choice = line.chars().nth(0).unwrap();
        let mut my_choice = line.chars().nth(2).unwrap();

        total_part1 +=
            part1::get_shape_value(my_choice) + part1::get_round_score(my_choice, opponent_choice);

        total_part2 += {
            my_choice = part2::get_shape_to_play_against_opponent_shape(opponent_choice, my_choice);

            part1::get_shape_value(my_choice) + part1::get_round_score(my_choice, opponent_choice)
        };
    });

    println!("> Day 2");

    println!("Part 1: Answer is {total_part1}");

    println!("Part 2: Answer is {total_part2}");
}
