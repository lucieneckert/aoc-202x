use crate::parse;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

type Point = (i32, i32, i32);

fn dist(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.0 - p2.0).unsigned_abs() as u64;
    let dy = (p1.1 - p2.1).unsigned_abs() as u64;
    let dz = (p1.2 - p2.2).unsigned_abs() as u64;
    (((dx * dx) + (dy * dy) + (dz * dz)) as f64).sqrt()
}

#[derive(Debug)]
struct UnionFind<T: Eq + Hash + Clone + Copy> {
    map: HashMap<T, T>,
}

impl<T: Eq + Hash + Clone + Copy> UnionFind<T> {
    fn new() -> Self {
        UnionFind {
            map: HashMap::new(),
        }
    }

    fn find(&self, k: &T) -> Option<&T> {
        let parent = self.map.get(k);
        match parent {
            None => None,
            Some(pk) => {
                if pk == k {
                    Some(pk)
                } else {
                    // Not condensing the paths here, since self is not mutable:
                    self.find(pk)
                }
            }
        }
    }

    fn union(&mut self, k1: T, k2: T) {
        let p1 = self.find(&k1).cloned();
        let p2 = self.find(&k2).cloned();
        // If neither in a set, use k1 as parent for both:
        if p1.is_none() && p2.is_none() {
            self.map.insert(k1.clone(), k1.clone());
            self.map.insert(k2, k1);
            return;
        }
        // If one is not in a set, set its parent to be the other's root:
        if p1.is_none() {
            self.map.insert(k1, p2.unwrap().clone());
            return;
        }
        if p2.is_none() {
            self.map.insert(k2, p1.unwrap().clone());
            return;
        }
        // Otherwise, default to uniting under p1
        let found_p1 = p1.unwrap().clone();
        let found_p2 = p2.unwrap().clone();
        if found_p1 == found_p2 {
            return;
        }
        self.map.insert(found_p2, found_p1);
    }

    // So, in hindsight, since we're just counting the length of the sets by
    // doing this, UnionFind is a bit overkill. Alas. Maybe there's some way to
    // keep track of the number of elements in each set as we build it up...
    // We'll just say this is all in the name of rust iterator practice :)
    fn count_sets(&self) -> Vec<usize> {
        let mut roots_to_set: HashMap<&T, usize> = HashMap::new();
        for k in self.map.keys() {
            let key_root = self.find(k).unwrap();
            let root_count = roots_to_set.get_mut(key_root);
            match root_count {
                None => {
                    roots_to_set.insert(key_root, 1);
                }
                Some(count) => *count += 1,
            };
        }

        roots_to_set.values().map(|v| *v).collect()
    }
}

pub(crate) fn solve(input: BufReader<File>, part: parse::Part) {
    let mut uf: UnionFind<Point> = UnionFind::new();

    let points = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let elems: Vec<&str> = l.split(",").collect();
            (
                elems[0].parse().unwrap(),
                elems[1].parse().unwrap(),
                elems[2].parse().unwrap(),
            )
        })
        .collect::<Vec<Point>>();

    let dist_to_points: Vec<(f64, (&Point, &Point))> = points
        .iter()
        .combinations(2)
        .map(|v| {
            let p1 = v[0];
            let p2 = v[1];
            let d = dist(p1, p2);
            (d, (p1, p2))
        })
        .filter(|(d, _)| *d != 0.0) // remove self edges
        .sorted_by(|(d1, _), (d2, _)| d1.total_cmp(d2))
        .collect();

    match part {
        parse::Part::Part1 => {
            // Note: for the example case, we take 10.
            for (_, (p1, p2)) in dist_to_points.iter().take(1000) {
                // println!("union {p1:?} and {p2:?}...");
                uf.union(**p1, **p2);
            }
            // Count the number of sets:
            let prod = uf
                .count_sets()
                .iter()
                .sorted_by_key(|size| **size as i32 * -1)
                .take(3)
                .fold(1_u64, |acc, size| acc * *size as u64);
            println!("output: {prod}")
        }
        parse::Part::Part2 => {
            // Let's keep unioning until we get a single set.
            // Hardly the most clever solution, but it will work...
            for (_, (p1, p2)) in dist_to_points.iter() {
                uf.union(**p1, **p2);
                let set_lens = uf.count_sets();
                if set_lens.len() == 1 && set_lens[0] == points.len() {
                    let prod = p1.0 as i64 * p2.0 as i64;
                    println!("output: {prod}");
                    return;
                }
            }
            panic!("we've failed to connect everything, somehow.")
        }
    }
}
