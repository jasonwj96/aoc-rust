use aoc_rust::year2024::day01;
use std::env;
use std::io::{self};


fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;
    println!("Current directory: {}", cwd.display());

    let input_path = cwd.join("input/year2024/day01.txt");

    let result = day01::part1(&input_path)?;
    println!("Part 1 result: {}", result);

    let result = day01::part2(&input_path)?;
    println!("Part 2 result: {}", result);

    Ok(())
}