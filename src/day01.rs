use std::io::{BufReader, BufRead};
use std::fs::File;

use crate::parse;

const DEBUG: bool = false;

fn parse_instruction(text: String) -> Instruction {
    let (dir, val) = text.split_at(1);
    Instruction{
        direction:  match dir {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("invalid direction {:}", dir))
        }.unwrap(),
        value:  val.parse().unwrap(),
    }
}

enum Direction {
    Left,
    Right
}
struct Instruction {
    direction: Direction,
    value: usize
}

pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    let mut final_count: i32 = 0;
    input
        .lines()
        .map_while(Result::ok)
        .map(parse_instruction)
        .fold(50, |acc, instruction| {
            let next = acc 
                + (instruction.value as i32 * (
                    if matches!(instruction.direction, Direction::Left) { -1 } else { 1 }
                ));
            match part {
                parse::Part::Part1 => {
                    if next % 100 == 0 {
                        final_count += 1;
                    }
                },
                parse::Part::Part2 => {
                    if acc != 0 && next <= 0 {
                        final_count += 1;
                    }
                    final_count += next.abs() / 100;
                }
            }
            let next = next.rem_euclid(100);
            if DEBUG {
                println!("line acc: {}", next);
                println!("final count: {}", final_count);
            }
            next
        });

    println!("output: {}", final_count)
}
