use std::io;
use std::iter::Peekable;

// This was from the approach that I wanted to do -- keep rotating the input
// and fold together the XMAS's on each line.
// Idk what the equivalent of numpy is for rust yet...
#[allow(dead_code)]
fn count_substr(s: String, sub: &str) -> usize {
    println!("s: {}", s);
    let mut count = 0;
    for i in 0..s.len() - sub.len() + 1 {
        let s2 = &s[i..i + sub.len()];
        println!("i: {}, s2: {}", i, s2);
        if *s2 == *sub {
            count += 1;
        }
    }
    return count;
}

type Position = (usize, usize);

fn add(pos: Position, step: &(i32, i32)) -> Option<Position> {
    let new_x = pos.0 as i32 + step.0;
    let new_y = pos.1 as i32 + step.1;
    if new_x < 0 || new_y < 0 {
        return None;
    }
    return Some((new_x as usize, new_y as usize));
}

struct WordSearch {
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        match self.grid.get(x as usize) {
            Some(row) => row.get(y as usize),
            None => None,
        }
    }

    fn count(&self, pos: Position, word: &str) -> usize {
        let all_dirs: [(i32, i32); 8] = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
        ];
        return all_dirs
            .iter()
            .map(|dir| {
                println!(
                    "Starting check at ({}, {}), dir ({}, {}), for {}",
                    pos.0, pos.1, dir.0, dir.1, word
                );
                let chars = word.chars().into_iter().peekable();
                return self.constructs_word(pos, dir, chars);
            })
            .filter(|b| *b)
            .count();
    }

    fn constructs_word(
        &self,
        pos: Position,
        step: &(i32, i32),
        mut remaining: Peekable<impl Iterator<Item = char>>,
    ) -> bool {
        println!("Checking ({}, {})", pos.0, pos.1);
        let c = self.get(pos.0, pos.1);
        if c.is_none() {
            // Position is invalid
            return false;
        }

        if *c.unwrap() != remaining.next().expect("missing remaining chars") {
            println!("Wrong char {}", self.get(pos.0, pos.1).expect(""));
            return false;
        }

        // If we've consumed the remaining characters, the word is constructed:
        if remaining.peek().is_none() {
            return true;
        }

        // Otherwise, see if we can construct the rest of the word after step:
        let next_pos = add(pos, step);
        return match next_pos {
            Some(p) => self.constructs_word(p, step, remaining),
            None => false,
        };
    }

    fn count_all(&self, word: &str) -> usize {
        return self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, _)| (x, y)))
            .filter(|(x, y)| {
                let c = self.get(*x, *y);
                return match c {
                    Some(c) => *c == 'X',
                    None => false,
                };
            })
            .map(|pos| self.count(pos, word))
            .sum();
    }
}

fn main() {
    let input_grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.expect("unexpected missing line").chars().collect())
        .collect();

    let word_search = WordSearch { grid: input_grid };

    println!("Output: {}", word_search.count_all("XMAS"));
}
