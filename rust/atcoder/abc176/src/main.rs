// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_traits::Pow;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::*;
use std::ops::Range;
use std::str::FromStr;
use superslice::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

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

lazy_static! {
    static ref H: Mutex<Vec<i32>> = Mutex::default();
    static ref W: Mutex<Vec<i32>> = Mutex::default();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
        // .reverse()
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//abc176-D
// #[fastout]
fn main() {
    input![
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        dH: usize,
        dW: usize,
        s: [Chars; h]
    ];

    let ch = ch - 1;
    let cw = cw - 1;
    let dH = dH - 1;
    let dW = dW - 1;

    let mut dp = vec![vec![None; w]; h];
    // dp[ch][cw] = Some(0);

    let mut magic_q = BinaryHeap::new();
    magic_q.push(State {
        cost: 0,
        pos: (ch, cw),
    });

    while dp[dH][dW] == None && !magic_q.is_empty() {
        let min_state = magic_q.pop().unwrap();
        // eprintln!("{:?}", min_state);

        if dp[min_state.pos.0][min_state.pos.1].is_some() {
            continue;
        }

        let mut q = VecDeque::new();
        q.push_back(min_state);

        while !dp[dH][dW].is_some() && !q.is_empty() {
            let next = q.pop_front().unwrap();

            let (n_h, n_w) = next.pos;

            if dp[n_h][n_w].is_some() {
                continue;
            }
            // eprintln!("next: {:?}", next);

            dp[n_h][n_w] = Some(next.cost);

            let walks = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (del_h, del_w) in walks.iter() {
                let new_h = n_h as i64 + del_h;
                let new_w = n_w as i64 + del_w;

                if new_h < 0 || new_w < 0 {
                    continue;
                }

                if new_h >= h as i64 || new_w >= w as i64 {
                    continue;
                }

                if s[new_h as usize][new_w as usize] == '#' {
                    continue;
                }

                if dp[new_h as usize][new_w as usize].is_some() {
                    continue;
                }

                q.push_back(State {
                    cost: next.cost,
                    pos: (new_h as usize, new_w as usize),
                });
            }

            let magics = [
                (-2, -2),
                (-2, -1),
                (-2, 0),
                (-2, 1),
                (-2, 2),
                //
                (-1, -2),
                (-1, -1),
                (-1, 1),
                (-1, 2),
                //
                (0, -2),
                (0, 2),
                //
                (1, -2),
                (1, -1),
                (1, 1),
                (1, 2),
                //
                (2, -2),
                (2, -1),
                (2, 0),
                (2, 1),
                (2, 2),
            ];

            for (del_h, del_w) in magics.iter() {
                let new_h = n_h as i64 + del_h;
                let new_w = n_w as i64 + del_w;

                if new_h < 0 || new_w < 0 {
                    continue;
                }

                if new_h >= h as i64 || new_w >= w as i64 {
                    continue;
                }

                if dp[new_h as usize][new_w as usize].is_some() {
                    continue;
                }

                if s[new_h as usize][new_w as usize] == '#' {
                    continue;
                }

                magic_q.push(State {
                    cost: next.cost + 1,
                    pos: (new_h as usize, new_w as usize),
                });
            }
        }
    }

    // .....
    // #####
    // ####.
    // .####
    // ..#.#
    // println!("{:?}", s);

    let a = dp[dH][dW];
    let ans = if a.is_some() { a.unwrap() as i64 } else { -1 };

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
