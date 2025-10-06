pub mod year2024;

pub trait Solution {
    fn parse(input: &str) -> Self
        where
            Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}