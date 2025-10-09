use aoc_rust::year2024::day02::*;
use std::env;

#[test]
fn part1_test() {
    let cwd = env::current_dir().expect("Invalid path.");

    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day02_test.txt");

    let input = parse(input_path).expect("Input was not parsed correctly.");

    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let cwd = env::current_dir().expect("Invalid path.");

    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day02_test.txt");

    let input = parse(input_path).expect("Input was not parsed correctly.");

    assert_eq!(part2(&input), 4);
}