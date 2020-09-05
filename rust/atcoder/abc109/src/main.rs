// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_traits::Pow;
use proconio::marker::Chars;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use itertools::concat;
// use itertools_num::*;
use lazy_static::lazy_static;
// use libm::*;
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

const MOD: usize = 1000_000_000 + 7;

//abc109-D
// #[fastout]
fn main() {
    input![h: usize, w: usize, mut a: [[u32; w]; h]];

    let h = h;
    let w = w;
    let a = a;

    let odd_count: usize = a
        .iter()
        .map(|a| {
            a.iter()
                .map(|a| if a % 2 == 1 { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    let odds: Vec<(usize, usize, u32)> = a
        .iter()
        .enumerate()
        .map(|(h, a)| {
            a.iter().enumerate().filter_map(move |(w, val)| {
                if val % 2 == 1 {
                    eprintln!("{:?}", (w, h, *val));
                    Some((w, h, *val))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    eprintln!("{:?}", odds);

    // let n = h * w - (odd_count % 2);
    let mut is_uses = vec![false; odds.len()];
    let mut is_visited = vec![vec![false; w]; h];

    let mut trackers = Vec::new();
    for (idx, (w, h, v)) in odds.iter().enumerate() {
        if odds.len() % 2 == 1
            && is_uses
                .iter()
                .map(|is_use| if *is_use { 0 } else { 1 })
                .sum::<usize>()
                == 1
        {
            continue;
        }

        if is_uses[idx] {
            continue;
        }

        let mut q = VecDeque::new();
        is_visited[*h][*w] = true;
        let mut tracker = vec![(w, h)];
        q.push_back(((w, h), tracker));

        while !q.is_empty() {
            let ((w, h), tracker) = q.pop_front().unwrap();
            if a[*h][*w] % 2 == 1 {
                let idx = odds
                    .iter()
                    .position(|(hh, ww, _)| hh == h && ww == w)
                    .unwrap();
                is_uses[idx] = true;
                trackers.push(tracker);
                break;
            }
        }
    }

    // println!("{}", n);
    // for

    // println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
