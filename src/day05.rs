use std::{io::{BufReader, BufRead}, fs::File};

use crate::parse;

type Bound = (i64, i64);

#[derive(Debug)]
struct Node {
  value: Bound,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
  is_lower: bool,
}

impl Node {
  fn get_cmp_val(&self) -> i64 {
    if self.is_lower { self.value.0 } else { self.value.1 }
  }

  fn find(&self, v: i64) -> bool {
    if self.value.0 <= v && self.value.1 >= v {
      return true
    } 
    let next = {
      let cmp_value = self.get_cmp_val();
      if cmp_value < v { &self.right } else { &self.left }
    };
    match next {
      None => false,
      Some(node) => node.find(v)
    }
  }

  fn add(&mut self, bound: Bound, lower: bool) {
    let b_val = if self.is_lower { bound.0 } else { bound.1 };
    let cmp_val = self.get_cmp_val();
    if cmp_val > b_val {
      match &mut self.left {
        None => self.left = Some(Box::from(Node::from_bound(bound, lower))),
        Some(b) => b.add(bound, lower),
      }
    } else if cmp_val < b_val {
      match &mut self.right {
        None => self.right = Some(Box::from(Node::from_bound(bound, lower))),
        Some(b) => b.add(bound, lower),
      }
    }
  }

  fn from_bound(bound: Bound, lower: bool) -> Node {
    Node { value: bound, left: None, right: None, is_lower: lower }
  }
}

#[derive(Debug)]
struct Tree {
  lower_root: Option<Node>,
  upper_root: Option<Node>
}

impl Tree {
  fn new() -> Tree {
    Tree { lower_root: None, upper_root: None }
  }

  fn add(&mut self, bound: Bound) {
    match &mut self.lower_root {
      Some(n) => n.add(bound, true),
      None => self.lower_root = Some(Node::from_bound(bound, true))
    }
    match &mut self.upper_root {
      Some(n) => n.add(bound, false),
      None => self.upper_root = Some(Node::from_bound(bound, false))
    }
  }

  fn find(&self, v: i64) -> bool {
    (match &self.lower_root {
      None => false,
      Some(n) => n.find(v)
    }) || match &self.upper_root {
      None => false,
      Some(n) => n.find(v)
    }
  }
}



pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
  let lines = input
    .lines()
    .map(|l| l.unwrap())
    .fold(String::new(), |acc, l| acc + l.as_str() + "\n");

  let (bounds, ids) = lines.split_once("\n\n").unwrap();
  // println!("bounds {bounds}, ids {ids}");

  if matches!(part, parse::Part::Part1) {
    let mut tree = Tree::new();
    for bound in bounds.lines() {
      let (low, high) = bound.split_once("-").unwrap();
      tree.add((low.parse().unwrap(), high.parse().unwrap()))
    }

    let fresh = ids.lines()
      .map(|id| id.parse().unwrap())
      .filter(|id| tree.find(*id))
      .inspect(|id| println!("{id} is fresh"))
      .count();

    println!("output: {fresh}");
  } else {
    let mut sorted = bounds
      .lines()
      .map(|b| b.split_once("-").unwrap())
      .map(|(low, high)| (low.parse().unwrap(), high.parse().unwrap()))
      .collect::<Vec<Bound>>();
    sorted.sort_by_key(|b| b.0);

    let mut finalized_bounds = Vec::new();
    let mut finalized_bound = (0, 0);
    for (idx, bound) in sorted.iter().enumerate() {
      if idx == 0 {
        finalized_bound = *bound;
        continue
      }
      if bound.0 <= finalized_bound.1 {
        finalized_bound.1 = if bound.1 > finalized_bound.1 { bound.1 } else { finalized_bound.1 }
      } else {
        println!("pushing {finalized_bound:?}");
        finalized_bounds.push(finalized_bound);
        finalized_bound = *bound
      }
    }
    finalized_bounds.push(finalized_bound);

    let sum: i64 = finalized_bounds.iter()
      .map(|(low, high)| high - low + 1)
      .inspect(|v| println!("bound has {v} ids"))
      .sum();

    println!("output: {sum}");
  }
  
}