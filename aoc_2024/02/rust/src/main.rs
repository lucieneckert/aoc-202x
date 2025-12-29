use std::env;
use std::io;

fn is_safe_diff(report: impl Iterator<Item = i32>) -> bool {
    let mut diffs = Vec::<i32>::new();
    let mut prev: Option<i32> = None;
    for num in report {
        match prev {
            Some(prev_num) => {
                diffs.push(num - prev_num);
            }
            None => {}
        }
        prev = Some(num);
    }

    // Check to see if the sequence is monotonic:
    let num_decreasing = diffs.iter().filter(|d| **d <= 0).count();
    let num_increasing = diffs.iter().filter(|d| **d >= 0).count();
    let is_monotonic = if num_increasing > num_decreasing {
        num_decreasing == 0
    } else {
        num_increasing == 0
    };

    // Check to see if we have any diffs out of range:
    let num_bad_jumps = diffs
        .iter()
        .map(|d| d.abs())
        .filter(|d| *d < 1 || *d > 3)
        .count();

    return is_monotonic && num_bad_jumps == 0;
}

#[allow(dead_code)]
fn is_safe_big_loop(report: impl Iterator<Item = i32>) -> bool {
    let mut decreasing: Option<bool> = None;
    let mut prev: Option<i32> = None;

    for num in report {
        match prev {
            Some(prev_num) => {
                // Check monotonicity of numbers:
                match decreasing {
                    Some(b) => {
                        if b != (num < prev_num) {
                            println!("not monotonic: {} -> {}", prev_num, num);
                            return false;
                        }
                    }
                    None => {
                        decreasing = if num != prev_num {
                            Some(num < prev_num)
                        } else {
                            None
                        }
                    }
                }

                // Check immediate difference between numbers:
                let diff = (prev_num - num).abs();
                if diff < 1 || diff > 3 {
                    println!("not safe jump: {} -> {}, ({})", prev_num, num, diff);
                    return false;
                }
                prev = Some(num);
            }
            None => prev = Some(num),
        }
    }
    return true;
}

fn main() {
    println!("AoC 2024 Day 2 - Rust");

    let mut args = env::args();
    let part = match args.nth(1).expect("expected part flag").parse::<usize>() {
        Ok(num) => num,
        Err(e) => panic!("unable to parse part flag: {:?}", e),
    };

    let solution: usize = io::stdin()
        .lines()
        .map(|s| {
            let line = s.expect("did not get expected line");
            let report = line
                .split(" ")
                .map(|e| e.parse::<i32>().expect("unable to parse line token as int"));
            return match part {
                1 => is_safe_diff(report),
                2 => {
                    // Split into all possible subsequences
                    for skip_idx in 0..report.clone().count() {
                        if is_safe_diff(
                            report
                                .clone()
                                .enumerate()
                                .filter(|(i, _)| *i != skip_idx)
                                .map(|(_, e)| e),
                        ) {
                            return true;
                        }
                    }
                    return false;
                }
                _ => panic!("unsupported part {}", part),
            };
        })
        .filter(|is_safe| *is_safe)
        .count();

    println!("Output: {}", solution);
}
