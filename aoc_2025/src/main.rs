use clap::Parser;
use std::{collections::HashMap, fs::File, io::BufReader};

mod parse;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

type Solver = fn(BufReader<File>, parse::Part);

#[derive(Parser)]
#[command(about)]
struct Args {
    // The day of the problem to solve.
    #[arg(long)]
    day: String,
    // The problem input.
    #[arg(long)]
    input_path: String,
    // The part (1 or 2) of the problem to solve.
    #[arg(long)]
    part: String,
}

fn main() {
    println!("📅 AoC 2025!");

    let solutions: HashMap<String, Solver> = HashMap::from([
        (String::from("1"), day01::solve as Solver),
        (String::from("2"), day02::solve as Solver),
        (String::from("3"), day03::solve as Solver),
        (String::from("4"), day04::solve as Solver),
        (String::from("5"), day05::solve as Solver),
        (String::from("6"), day06::solve as Solver),
    ]);

    let args = Args::parse();
    println!(
        "📩 Using input at path {}, for day {}, part {}\n",
        args.input_path, args.day, args.part
    );
    let input = parse::get_input(args.input_path).unwrap();

    let solve = solutions
        .get(&args.day)
        .expect("no solver implemented for day!");

    solve(input, parse::Part::from(args.part))
}
