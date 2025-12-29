use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use crate::parse;

#[derive(Debug)]
struct Bank{
    batteries: Vec<u8>,
    cache: HashMap<(u32, usize), u64>
}

impl Bank {
    fn max_joltage(&self, left: Option<usize>, right: Option<usize>) -> u64 {
        match (left, right) {
            (Some(l), Some(r)) => (self.batteries.get(l).unwrap() * 10 + self.batteries.get(r).unwrap()).into(),
            (Some(l), None) => {
                (l+1..self.batteries.len())
                .map(|r| self.max_joltage(left, Some(r)))
                .max()
                .unwrap()
            }
            _ => {
            (0..self.batteries.len() - 1)
                .map(|l| self.max_joltage(Some(l), None))
                .max()
                .unwrap()
            }
        }
    }

    fn max_joltage_for_digits(&mut self, digits: u32, start_idx: usize) -> u64 {
        if digits == 0 {
            return 0
        }
        
        let key = &(digits, start_idx);
        if self.cache.contains_key(key) {
            return *self.cache.get(key).unwrap()
        }

        let max_joltage = (start_idx..=self.batteries.len() - digits as usize)
            .map(|i| {
                let digit: u64 = (*self.batteries.get(i).unwrap()).into();
                let delta = digit * 10_u64.pow(digits - 1);
                self.max_joltage_for_digits(
                    digits - 1,
                    i + 1
                ) + delta
            })
            .max();
        let out = max_joltage.unwrap_or(0);
        self.cache.insert(*key, out);
        out
    }
}

impl From<String> for Bank {
    fn from(value: String) -> Self {
        Bank {
            batteries: value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
            cache: HashMap::new()
        }
    }
}


pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    let sum: u64 = input
        .lines()
        .map(|l| l.unwrap())
        .map(Bank::from)
        .map(|mut b| {
            match part {
                parse::Part::Part1 => b.max_joltage(None, None),
                parse::Part::Part2 => b.max_joltage_for_digits(12, 0)
            }
        })
        .inspect(|j| println!("joltage: {}", j))
        .sum();

    println!("\noutput: {}", sum)
}