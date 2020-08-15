// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::BinaryHeap;
use std::io::*;
use std::ops::Range;
use std::str::FromStr;
use superslice::*;

use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit;

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

// #[fastout]
fn main() {
    // Create an undirected graph with `i32` nodes and edges with `()` associated data.
    // let mut g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);
    let mut g = UnGraph::new_undirected();

    let nodes: Vec<_> = Range { start: 0, end: 10 }
        .map(|i: i32| g.add_node(i.to_string()))
        .collect();

    let _: Vec<_> = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6), (0, 9)]
        .iter()
        .map(|(f, t)| {
            g.add_edge(nodes[*f], nodes[*t], 1);
        })
        .collect();
    // let n = g.add_node(5);
    // g.add_edge(1.into(), 5.into(), ());

    let mut dfs = visit::Bfs::new(&g, nodes[0]);

    while let Some(nx) = dfs.next(&g) {
        // let a = g;
        println!("{:?}", nx);
    }

    println!("{:?}", g);

    let node_map = dijkstra(&g, nodes[0], Some(nodes[5]), |_| 1);
    node_map.iter().for_each(|item| {
        println!("{:?}", item);
    });

    // // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
    // let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
    // assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());

    // // Get the minimum spanning tree of the graph as a new graph, and check that
    // // one edge was trimmed.
    // let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
    // assert_eq!(g.raw_edges().len() - 1, mst.raw_edges().len());
}
