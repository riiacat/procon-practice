// -*- coding:utf-8-unix -*-

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::io::*;
use std::str::FromStr;
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

//abc075-C
#[fastout]
fn main() {
    input![n: usize, m: usize, edge: [(usize, usize); m]];

    // println!("n={}, m={} edges:{:?}", n, m, edge);

    let mut ad_mat = vec![vec![false; n as usize]; n as usize];

    for (from, to) in edge.iter() {
        ad_mat[from - 1][to - 1] = true;
        ad_mat[to - 1][from - 1] = true;
    }

    // println!("{:?}", ad_mat);

    let dfs = |non_use_edge: (usize, usize)| {
        let (non_from, non_to) = non_use_edge;
        let (non_from, non_to) = (non_from - 1, non_to - 1);

        // eprintln!("non_use_edge: {}, {}", non_from, non_to);
        for from_node_idx in 0..n {
            // eprintln!("from_node_idx: {:?}", from_node_idx);
            let mut s = Vec::new();
            let mut is_visits = vec![false; n];
            s.push(from_node_idx);

            let is_all_visits = |is_visits: &Vec<bool>| {
                for is_visit in is_visits.iter() {
                    // eprintln!("is_visit: {}", *is_visit);

                    if *is_visit {
                    } else {
                        return false;
                    }
                }

                return true;
            };

            while !(s.is_empty() || is_all_visits(&is_visits)) {
                let n = s.pop().unwrap();

                if is_visits[n] {
                    continue;
                }

                is_visits[n] = true;
                for (idx, to) in ad_mat[n].iter().enumerate() {
                    if (non_from == n && non_to == idx) || (non_from == idx && non_to == n) {
                        // eprintln!("non_from: {}, non_to == {}", non_from, non_to);

                        continue;
                    }

                    if *to {
                        if !is_visits[idx] {
                            s.push(idx);
                        }
                    }
                }
            }

            // eprintln!(
            //     "is_all_visits: {}, is_visits: {:?}, ",
            //     is_all_visits(&is_visits),
            //     is_visits
            // );

            if !is_all_visits(&is_visits) {
                return false;
            }
        }

        return true;
    };

    let mut ans = 0;
    for non_use_edge in edge.iter() {
        if !dfs(*non_use_edge) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
