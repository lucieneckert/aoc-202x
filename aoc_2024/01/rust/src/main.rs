use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::io;
use std::iter;

fn collect_heap<T: Ord>(mut heap: BinaryHeap<T>) -> Vec<T> {
    let mut out = Vec::<T>::new();
    while !heap.is_empty() {
        match heap.pop() {
            Some(elem) => out.push(elem),
            None => panic!("heap.pop() with empty heap"),
        }
    }
    return out;
}

fn main() {
    println!("AoC 2024 Day 1 - Rust");

    // Parse input lines and construct a list of distance pairs
    let lines = io::stdin().lines();
    let pairs = lines
        .map(|line| {
            return line.expect("no line in input");
        })
        .map(|line| {
            let mut elems = line.split(' ');
            let left = elems
                .next()
                .expect("no tokens found in input line")
                .parse::<i32>()
                .expect("unable to parse first token as i32");
            let right = elems
                .last()
                .expect("no final token found in input line")
                .parse::<i32>()
                .expect("unable to parse last token as i32");
            return (left, right);
        });

    // Sort them so we can compare --
    // Max heap by default, but this is fine as long as they line up
    let mut left_heap = BinaryHeap::<i32>::new();
    let mut right_heap = BinaryHeap::<i32>::new();
    for (l, r) in pairs {
        left_heap.push(l);
        right_heap.push(r);
    }
    let left_sorted = collect_heap(left_heap);
    let right_sorted = collect_heap(right_heap);

    let args: Vec<String> = env::args().collect();
    let part = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(e) => panic!("unable to parse part flag: {:?}", e),
    };
    let solve: fn(Vec<i32>, Vec<i32>) -> i32 = match part {
        1 => part_one,
        2 => part_two,
        _ => panic!("invalid part (must be 1 or 2)"),
    };

    let output = solve(left_sorted, right_sorted);
    println!("Output: {}", output);
}

fn part_one(left: Vec<i32>, right: Vec<i32>) -> i32 {
    return iter::zip(left, right).map(|(l, r)| (l - r).abs()).sum();
}

fn part_two(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let right_counts = right
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut map, num| {
            let count = match map.get(num) {
                Some(c) => *c,
                None => 0,
            };
            map.insert(*num, count + 1);
            return map;
        });
    println!("right counts: {:?}", right_counts);
    return left
        .iter()
        .map(|num| {
            let count = match right_counts.get(num) {
                Some(c) => *c,
                None => 0,
            };
            return num * count as i32;
        })
        .sum();
}
