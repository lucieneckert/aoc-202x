use std::{io::{BufReader, BufRead}, fs::File};

use crate::parse;

#[derive(Debug)]
struct Grid {
    elems: Vec<Vec<bool>>
}

impl Grid {
    fn roll_at(&self, (x, y): (usize, usize)) -> bool {
        if x >= self.elems.len() {
            return false
        }
        let row = self.elems.get(x).unwrap();
        if y >= row.len() {
            return false
        }
        *row.get(y).unwrap()
    }

    fn count_adjacent(&self, (x, y): (usize, usize)) -> usize {
        (-1isize..=1)
            .map(|dx| (-1isize..=1)
                .filter(|dy| !(dx == 0_isize && *dy == 0_isize))
                .map(|dy| (x as isize + dx, y as isize + dy))
                .filter_map(|(x, y)| {
                    let ux: Option<usize> = x.try_into().ok();
                    let uy: Option<usize> = y.try_into().ok();
                    match (ux, uy) {
                        (Some(x), Some(y)) => Some((x, y)),
                        _ => None
                    }
                })
                .filter(|(x, y)| self.roll_at((*x, *y)))
                // .inspect(|c| println!("\tcoord: {c:?}"))
                .count()
            )
            .sum()
    }

    fn get_accessible(&self, max_adjacent: usize) -> Vec<(usize, usize)> {
        self
            .elems
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, is_roll)| **is_roll)
                    // .inspect(|(y, _)| println!("counting for {:?}", (x, y)))
                    .filter(|(y, _)| self.count_adjacent((x, *y)) < max_adjacent)
                    // .inspect(|e| println!("take {e:?}"))
                    .map(|(y, _)| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }

    fn remove_rolls(mut self, coords: Vec<(usize, usize)>) -> Grid {
        for (x, y) in coords {
            self.elems[x][y] = false;
        }
        self
    }
}

impl From<BufReader<File>> for Grid {
    fn from(value: BufReader<File>) -> Self {
        Grid {
            elems: value.lines()
            .map(|line| line.unwrap())
            .map(|line| line
                .chars()
                .map(|c| c == '@')
                .collect()
            )
            .collect()
        }
    }
}

pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    let mut grid: Grid = input.into();
    // println!("grid: {grid:?}");
    let output: usize = match part {
        parse::Part::Part1 => grid.get_accessible(4).len(),
        parse::Part::Part2 => {
            let mut sum = 0_usize;
            loop {
                let coords = grid.get_accessible(4);
                if coords.is_empty() {
                    break;
                }
                sum += coords.len();
                grid = grid.remove_rolls(coords);
            }
            sum
        }
    };

    println!("output: {}", output);
}
 