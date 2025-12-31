use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parse;

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

impl From<&char> for Operation {
    fn from(value: &char) -> Self {
        match value {
            '+' => Operation::Add,
            '*' => Operation::Mult,
            _ => panic!("invalid operator string"),
        }
    }
}

#[derive(Debug)]
struct Problem {
    elements: Grid<char>,
    operation: Operation,
}

impl Problem {
    fn get_horizontal_numbers(&self) -> Vec<i64> {
        let mut out = Vec::new();
        for col in self.elements.iter_cols() {
            let num_str: String = col.filter(|c| **c != ' ').collect();
            out.push(num_str.parse().unwrap());
        }
        out
    }

    fn get_vertical_numbers(&self) -> Vec<i64> {
        let mut out = Vec::new();
        for row in self.elements.iter_rows() {
            let num_str: String = row.filter(|c| **c != ' ').collect();
            out.push(num_str.parse().unwrap());
        }
        out
    }

    // solve computes the answer to the Problem, given the provided rules for
    // part 1 vs 2.
    fn solve(self, part: &parse::Part) -> i64 {
        let init = match self.operation {
            Operation::Add => 0,
            Operation::Mult => 1,
        };
        let elements = match part {
            parse::Part::Part1 => self.get_horizontal_numbers(),
            parse::Part::Part2 => self.get_vertical_numbers(),
        };
        elements.iter().fold(init, |acc, e| match self.operation {
            Operation::Add => acc + e,
            Operation::Mult => acc * e,
        })
    }
}

// problems_from consumes a Grid of arranged input characters and produces a
// list of Problems that can be solved using the rules of cephalopod math, and
// either part 1 or 2 notation.
fn problems_from(arranged_input: Grid<char>) -> Vec<Problem> {
    let col_len = arranged_input.rows() - 1; // don't count the operator row
    let mut problems: Vec<Problem> = Vec::new(); // push problems here
    let mut current_grid: Vec<char> = Vec::new(); // current sub-Grid
    let mut last_seen_operation: Option<Operation> = None; // current Operation

    // Generally, the idea here is to go through each column of the input, and
    // split the Grid into new sub-Grids delimited by each column of whitespace.
    // It's a little convoluted:
    for col in arranged_input.iter_cols() {
        // If this is unset, we need to get the operation from the bottom row:
        if last_seen_operation.is_none() {
            last_seen_operation = Some(col.clone().last().unwrap().into());
        }
        // If we got a column of all whitespace, it's a problem boundary: send
        // along the current subgrid (via clone, I'm tired) and Operation:
        if col.clone().into_iter().all(|c| *c == ' ') {
            problems.push(Problem {
                elements: Grid::from_vec(current_grid.clone(), col_len),
                operation: last_seen_operation.unwrap(),
            });
            // Reset the mutable tracking variables &c:
            current_grid.clear();
            last_seen_operation = None;
            continue;
        }
        current_grid.extend(col.take(col_len));
    }
    // Push the final problem, if we've still got one (operation is Some):
    if last_seen_operation.is_some() {
        problems.push(Problem {
            elements: Grid::from_vec(current_grid, col_len),
            operation: last_seen_operation.unwrap(),
        });
    }
    problems
}

pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    // We're going to wrangle everything into a nice padded Grid, through the
    // following steps.
    // First: vectorize the input lines by character:
    let mut vec_input = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    // Next, using the maximum line length, pad out each line so that we can
    // rely on a fixed length (this is why it's mutable)
    let line_len = vec_input.iter().map(|l| l.len()).max().unwrap();
    for l in vec_input.iter_mut() {
        l.resize(line_len, ' ');
    }
    // Finally, we're going to flatten everything and feed it into a Grid, using
    // that max line length as the row length.
    let flattened_input = vec_input.into_iter().flatten().collect();
    let arranged_input = Grid::from_vec(flattened_input, line_len);

    // Now, we can parse out problems from that grid, and solve them:
    let problems = problems_from(arranged_input);
    let out: i64 = problems.into_iter().map(|p| p.solve(&part)).sum();

    println!("output: {out:?}")
}
