use aoc_rust::year2024::day01::*;
use std::env;


fn main() {
    let cwd = env::current_dir().expect("Invalid path.");
    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day01.txt");
    let input = parse(input_path).expect("Input was not parsed correctly.");

    println!("Part 1: {:?}", part1(&input).unwrap());
    println!("Part 2: {:?}", part2(&input).unwrap());
}