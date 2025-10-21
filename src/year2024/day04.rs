use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn dx(&self) -> isize {
        match self {
            Direction::UpLeft | Direction::Up | Direction::UpRight => -1,
            Direction::Left | Direction::Right => 0,
            Direction::DownLeft | Direction::Down | Direction::DownRight => 1,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Direction::UpLeft | Direction::Left | Direction::DownLeft => -1,
            Direction::Up | Direction::Down => 0,
            Direction::UpRight | Direction::Right | Direction::DownRight => 1,
        }
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        output.push(line?);
    }

    Ok(output)
}

fn count_xmas(grid: &[Vec<char>], x: usize, y: usize) -> u32 {
    let word = "XMAS";
    let mut count = 0;

    for &dir in &DIRECTIONS {
        let mut valid = true;
        for (i, c) in word.chars().enumerate() {
            let nx = x as isize + i as isize * dir.dx();
            let ny = y as isize + i as isize * dir.dy();
            if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
                valid = false;
                break;
            }
            if grid[nx as usize][ny as usize] != c {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1;
        }
    }

    count
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 'X' {
                count += count_xmas(&grid, x, y);
            }
        }
    }

    count
}

pub fn part2(lines: &Vec<String>) -> u32 {
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut count = 0;

    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == 'A' {
                let diag1 = (grid[x - 1][y - 1], grid[x + 1][y + 1]);
                let diag2 = (grid[x - 1][y + 1], grid[x + 1][y - 1]);
                if matches!(diag1, ('M', 'S') | ('S', 'M')) &&
                    matches!(diag2, ('M', 'S') | ('S', 'M')) {
                    count += 1;
                }
            }
        }
    }

    count
}