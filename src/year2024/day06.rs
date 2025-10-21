use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir { North, East, South, West }

impl Dir {
    fn turn_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
    fn delta(self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }
}

type Grid = Vec<Vec<char>>;

#[derive(Clone)]
pub struct Input {
    grid: Grid,
    start: (usize, usize),
}

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Input> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();
    let mut start = None;

    for (r, line) in reader.lines().enumerate() {
        let l = line?;
        let row: Vec<char> = l.chars().collect();
        for (c, &ch) in row.iter().enumerate() {
            if ch == '^' {
                start = Some((r, c));
            }
        }
        grid.push(row);
    }

    Ok(Input {
        grid,
        start: start.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No start found"))?,
    })
}

fn simulate(grid: &Grid, start: (usize, usize)) -> (HashSet<(usize, usize)>, bool) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<((usize, usize), Dir)> = HashSet::new();

    let mut pos = start;
    let mut dir = Dir::North;

    loop {
        visited.insert(pos);

        let state = (pos, dir);
        if seen.contains(&state) {
            return (visited, true); // loop
        }
        seen.insert(state);

        let (dr, dc) = dir.delta();
        let nr = pos.0 as isize + dr;
        let nc = pos.1 as isize + dc;

        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
            return (visited, false); // exit
        }
        let (nr, nc) = (nr as usize, nc as usize);

        if grid[nr][nc] == '#' {
            dir = dir.turn_right();
        } else {
            pos = (nr, nc);
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let (visited, _) = simulate(&input.grid, input.start);
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.grid.clone();
    let (visited, _) = simulate(&grid, input.start);
    let mut loops = 0;

    for &(r, c) in &visited {
        if (r, c) == input.start || grid[r][c] != '.' {
            continue;
        }
        grid[r][c] = '#';
        let (_, did_loop) = simulate(&grid, input.start);
        if did_loop {
            loops += 1;
        }
        grid[r][c] = '.';
    }

    loops
}
