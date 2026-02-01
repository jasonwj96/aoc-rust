use aoc_rust::year2025;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    year: u32,

    #[arg(long)]
    day: u32,
}

fn main() {
    let args = Args::parse();

    let cwd = std::env::current_dir().expect("Invalid path");
    let input_path =
        cwd.join(format!("input/year{}/day{:02}.txt", args.year, args.day));

    match (args.year, args.day) {
        (2025, 1) => {
            let input = year2025::day01::parse(input_path).unwrap();
            println!("Part 1: {}", year2025::day01::part1(&input));
            println!("Part 2: {}", year2025::day01::part2(&input));
        }
        _ => panic!("Solution not implemented."),
    }
}