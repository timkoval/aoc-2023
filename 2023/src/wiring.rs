use itertools::Itertools;
use rand::{self, seq::SliceRandom};

use common::prelude::*;

const SUCCESS: &str = "Global snow production restarted!
Now, how am I gonna go down safely to enjoy all this snow?!";

#[derive(Debug, Clone)]
struct Graph {
    counts: Vec<Option<u32>>,
    edges: Vec<[u32; 2]>,
    num_nodes: u32,
}

pub fn solve(part: Part) -> Result<String> {
    let input = include_str!("../inputs/wiring.txt");
    if part.two() {
        return Ok(SUCCESS.to_owned());
    }
    let graph: Graph = input.parse()?;
    Ok(loop {
        let mut alt = graph.clone();
        alt.fast_min_cut();
        if alt.num_edges() == 3 {
            let (a, b) = alt.two_counts().expect("no two counts");
            #[cfg(debug_assertions)]
            println!("{a} * {b}");
            break a * b;
        }
        #[cfg(debug_assertions)]
        println!("tick!");
    }
    .to_string())
}

impl Graph {
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn contract(&mut self, threshold: u32) {
        let mut rng = rand::thread_rng();
        while self.num_nodes > threshold {
            let [src, dst] = *self.edges.choose(&mut rng).expect("no edges");
            self.edges.retain_mut(|[a, b]| {
                if *a == dst {
                    *a = src;
                } else if *b == dst {
                    *b = src;
                }
                a != b
            });
            let new_count = self.counts[dst as usize].take().expect("dead dst");
            *self.counts[src as usize].as_mut().expect("dead src") += new_count;
            self.num_nodes -= 1;
        }
    }

    fn fast_min_cut(&mut self) {
        if self.num_nodes <= 6 {
            self.contract(2);
        } else {
            let threshold = 1 + (f64::from(self.num_nodes) / f64::sqrt(2.0)).ceil() as u32;
            let mut alt = self.clone();
            self.contract(threshold);
            self.fast_min_cut();
            if self.num_edges() == 3 {
                return;
            }
            alt.contract(threshold);
            alt.fast_min_cut();
            if alt.num_edges() < self.num_edges() {
                *self = alt;
            }
        }
    }

    fn two_counts(&self) -> Option<(u32, u32)> {
        self.counts.iter().copied().flatten().collect_tuple()
    }
}

impl std::str::FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let edges: Vec<_> = s
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .map(|(src, dsts)| dsts.split_whitespace().map(move |dst| [src, dst]))
                    .context("no colon delimeter")
            })
            .flatten_ok()
            .try_collect()?;
        let names = edges
            .iter()
            .flatten()
            .sorted()
            .dedup()
            .copied()
            .collect_vec();
        let edges = edges
            .into_iter()
            .map(|edge| {
                edge.map(|name| {
                    names
                        .iter()
                        .position(|&n| n == name)
                        .expect("names have it all") as u32
                })
            })
            .collect();
        Ok(Self {
            num_nodes: names.len().try_into()?,
            counts: vec![Some(1); names.len()],
            edges,
        })
    }
}
