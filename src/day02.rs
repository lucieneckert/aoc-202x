use std::{io::{BufReader, Read}, fs::File};

use crate::parse;

fn check_invalid_one(num: usize) -> bool {
    let digits = num.checked_ilog10().unwrap() + 1;
    let divisor = 10_usize.pow(digits / 2);
    digits.is_multiple_of(2) && num / divisor == num % divisor
}

// a = 123123123
// 123000000 + 000123000 + 000000123
// a = 123 * 10^6 + 123 * 10^3 + 123
fn check_invalid_two(num: usize) -> bool {
    let digits = num.checked_ilog10().unwrap() + 1;
    for length in 1..digits / 2 + 1 {
        if !digits.is_multiple_of(length) {
            continue
        }
        let mut nums = vec![];
        for i in 0..(digits / length) {
            let small = 10_usize.pow(i * length);
            nums.push((num / small) % 10_usize.pow(length));
        }
        let head = nums.first().unwrap();
        if nums.iter().all(|num| num == head) {
            return true
        }
    }
    false
}

fn sum_invalid(bounds: (&str, &str), check: fn(i: usize) -> bool) -> usize {
    let parse_bound = |b: &str| b
        .trim()
        .parse::<usize>()
        .expect("can't parse bound");
    let low = parse_bound(bounds.0);
    let high = parse_bound(bounds.1);

    let mut sum = 0;
    for i in low..high+1 {
        if check(i) {
            sum += i;
        }
    };

    sum
}

pub(crate) fn solve(mut input: BufReader<File>, part: parse::Part) {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let check = match part {
        parse::Part::Part1 => check_invalid_one,
        parse::Part::Part2 => check_invalid_two
    };

    let sum: usize = buf
        .split(",")
        .map(|elem| elem.split_once("-").unwrap())
        .map(|bounds| sum_invalid(bounds, check))
        .sum();

    println!("output: {}", sum)
}
