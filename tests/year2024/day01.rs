use aoc_rust::year2024::day01::{
    parse,
    part1,
    part2};

use std::env;


#[test]
fn part1_test() {
    let cwd = env::current_dir().expect("Invalid path.");

    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day01_test.txt");

    let input = parse(input_path).expect("Input was not parsed correctly.");

    assert_eq!(part1(&input).unwrap(), 11);
}

#[test]
fn part2_test() {
    let cwd = env::current_dir().expect("Invalid path.");

    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day01_test.txt");

    let input = parse(input_path).expect("Input was not parsed correctly.");

    assert_eq!(part2(&input).unwrap(), 31);
}