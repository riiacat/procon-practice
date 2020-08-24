// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_traits::Pow;
use petgraph::algo::dijkstra;
use petgraph::Directed;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use ascii::{AsciiChar, AsciiString};
use itertools::concat;
use lazy_static::lazy_static;
use libm::*;
use petgraph::Graph;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::*;
use std::ops::Range;
use std::str::FromStr;
use std::sync::Mutex;
use superslice::*;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

// lazy_static! {
//     static ref H: Mutex<Vec<i32>> = Mutex::default();
//     static ref W: Mutex<Vec<i32>> = Mutex::default();
// }

//abc079-D
// #[fastout]
fn main() {
    input![h: usize, w: usize, c: [[i64; 10]; 10], a: [[i64; w]; h]];

    // Graph::
    let mut edges = Vec::new();

    for i in 0..10 {
        for j in 0..10 {
            //revert edge
            edges.push((j, i, c[i][j]));
        }
    }

    // N, E, Ty(directed or undirected), Ix(size))
    let g: Graph<usize, _, Directed, _> = Graph::from_edges(edges.into_iter());
    // eprintln!("{:?}", g);

    let index = g
        .node_indices()
        .find(|i| {
            // eprintln!("{:?}: {}", i, g[*i]);
            i.index() == 1
        })
        .unwrap();

    let shortest_paths = dijkstra(&g, index, None, |a| *a.weight());

    let mut ans = 0;
    for hh in 0..h {
        for ww in 0..w {
            if a[hh][ww] != -1 {
                let index = g
                    .node_indices()
                    .find(|i| i.index() == a[hh][ww] as usize)
                    .unwrap();

                ans += shortest_paths.get(&index).unwrap();
            }
        }
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
