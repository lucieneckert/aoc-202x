use crate::parse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Manifold {
    t: usize, // number of times the manifold was stepped
    rows: Vec<Vec<char>>,
    num_splits: usize, // count of observed beam splits at time t
}

impl Manifold {
    fn step(&mut self) {
        if self.is_done() {
            return;
        };
        // based on the constraints, to compute state for the next t, we look at
        // the current t row and update the row t+1 based on its surroundings:
        let prev_row = self.rows.get(self.t).unwrap().clone();
        let next_row = self.rows.get_mut(self.t + 1).unwrap();
        for i in 0..next_row.len() {
            let c = next_row[i];
            match c {
                '.' => {
                    if prev_row[i] == '|' || prev_row[i] == 'S' {
                        next_row[i] = '|'
                    }
                }
                '^' => {
                    if prev_row[i] == '|' {
                        self.num_splits += 1;
                        if i > 0 {
                            next_row[i - 1] = '|';
                        }
                        if i < next_row.len() - 1 {
                            next_row[i + 1] = '|';
                        }
                    }
                }
                _ => continue,
            };
        }

        self.t += 1;
    }

    fn is_done(&self) -> bool {
        self.t >= self.rows.len() - 1
    }

    fn print(&self) {
        let t = self.t;
        println!("Manifold at t={t}");
        for row in &self.rows {
            let repr: String = row.iter().collect();
            println!("{repr}")
        }
        let splits = self.num_splits;
        println!("Total observed splits: {splits}\n")
    }

    fn count_timelines(&self) -> u64 {
        let mut downstream_cache: HashMap<(usize, usize), u64> = HashMap::new();
        let start_col = self.rows[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == 'S')
            .unwrap()
            .0;
        self._count_timelines((0, start_col), &mut downstream_cache)
    }

    fn _count_timelines(
        &self,
        pos: (usize, usize),
        downstream_cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if downstream_cache.contains_key(&pos) {
            return *downstream_cache.get(&pos).unwrap();
        }

        // If we're on the last row, we've reached the end of a timeline:
        if pos.0 >= self.rows.len() {
            return 1;
        }

        // Otherwise, sum downstream timelines based on the current char:
        let downstream_timelines = match self.rows[pos.0][pos.1] {
            '^' => {
                let mut sum = 0;
                if pos.1 > 0 {
                    sum += self._count_timelines((pos.0, pos.1 - 1), downstream_cache);
                }
                if pos.1 < self.rows[pos.0].len() - 1 {
                    sum += self._count_timelines((pos.0, pos.1 + 1), downstream_cache);
                }
                sum
            }
            _ => self._count_timelines((pos.0 + 1, pos.1), downstream_cache),
        };
        downstream_cache.insert(pos, downstream_timelines);
        downstream_timelines
    }
}

pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    let mut manifold = Manifold {
        t: 0,
        rows: input
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect(),
        num_splits: 0,
    };

    match part {
        parse::Part::Part1 => {
            while !manifold.is_done() {
                manifold.step();
                manifold.print();
            }
        }
        parse::Part::Part2 => {
            let sum = manifold.count_timelines();
            println!("output: {sum}");
        }
    }
}
