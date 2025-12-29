use regex::{Captures, Regex};
use std::env;
use std::io;

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

struct State {
    enabled: bool,
    sum: i32,
}

// get_mul_arg extracts the argument at position idx from the mul
// instruction's capture group.
fn get_mul_arg(c: &Captures, idx: usize) -> i32 {
    return c
        .get(idx)
        .expect("unable to get mul arg match from capture group")
        .as_str()
        .parse::<i32>()
        .expect("unable to parse mul arg as int");
}

// INSTRUCTION_RE matches capture groups like the following:
// * mul(a, b): for identifying the instruction to multiply a * b and add it to sum
// * do(): for identifying the do/enable instruction
// * don't(): for identifying the don't/disable instruction
const INSTRUCTION_RE: &str = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";

fn main() {
    let re = Regex::new(INSTRUCTION_RE).expect("unable to compile regex");

    let mut args = env::args();
    let part = match args.nth(1).expect("expected part flag").parse::<usize>() {
        Ok(num) => num,
        Err(e) => panic!("unable to parse part flag: {:?}", e),
    };

    let input = io::stdin()
        .lines()
        .map(|l| l.expect("missing expected line value"))
        .collect::<Vec<String>>()
        .join("\n");

    let solution: i32 = re
        .captures_iter(input.as_str())
        .map(|c| {
            match c
                .get(0)
                .expect("missing expected first capture group")
                .as_str()
            {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => Instruction::Mul(get_mul_arg(&c, 1), get_mul_arg(&c, 2)),
            }
        })
        .filter(|i| part == 2 || matches!(i, Instruction::Mul(_, _)))
        .fold(
            State {
                enabled: true,
                sum: 0,
            },
            |acc: State, instruction| match instruction {
                Instruction::Mul(a, b) => State {
                    enabled: acc.enabled,
                    sum: acc.sum + (if acc.enabled { a * b } else { 0 }),
                },
                Instruction::Do => State {
                    enabled: true,
                    sum: acc.sum,
                },
                Instruction::Dont => State {
                    enabled: false,
                    sum: acc.sum,
                },
            },
        )
        .sum;

    println!("Output: {}", solution);
}
