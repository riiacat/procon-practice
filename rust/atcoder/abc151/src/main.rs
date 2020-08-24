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
// use std::convert::TryInto;
use ascii::{AsciiChar, AsciiString};
use itertools::concat;
use lazy_static::lazy_static;
use libm::*;
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

//abc147-C
// #[fastout]
fn main() {
    input![h: usize, w: usize, s: [String; h]];

    let s: Vec<_> = s
        .into_iter()
        .map(|s: String| AsciiString::from_str(&s[..]).unwrap())
        .collect();

    let mut maze = unsafe { vec![vec![AsciiChar::from_ascii_unchecked('#' as u8); w + 2]; h + 2] };
    // eprintln!("before init");

    // let mut is_visit = vec![vec![false; w]; h];
    for hh in 1..(h + 1) {
        for ww in 1..(w + 1) {
            maze[hh][ww] = s[hh - 1][ww - 1];
            if maze[hh][ww] == '#' {
                // is_visit[hh - 1][ww - 1] = true;
            }
        }
    }

    // eprintln!("after init");
    let can_moves: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut ans = -1;
    for hh in 1..(h + 1) {
        for ww in 1..(w + 1) {
            if maze[hh][ww] == '#' {
                continue;
            }

            // let mut is_visit: Vec<_> = is_visit.iter().map(|v| v.clone()).collect();
            let mut dist = vec![vec![1000; w]; h];
            let mut new_ans = -1;
            //hh, wwをスタートにして、幅優先探索をする
            let mut q = VecDeque::new();
            q.push_back((hh, ww, 0));
            // while !(q.is_empty() || is_visit.iter().all(|row| row.iter().all(|a| *a))) {
            while !(q.is_empty()) {
                let (hh, ww, d) = q.pop_front().unwrap();
                // is_visit[hh - 1][ww - 1] = true;
                dist[hh - 1][ww - 1] = d;
                // eprintln!("{:?}", ((hh, ww), d));
                new_ans = max(new_ans, d);

                for (del_h, del_w) in &can_moves {
                    let new_h = (hh as i64 + del_h) as usize;
                    let new_w = (ww as i64 + del_w) as usize;

                    // eprintln!("{}, {}, {}", new_h, new_w, maze[new_h][new_w]);
                    if maze[new_h][new_w] == '#' || dist[new_h - 1][new_w - 1] < 1000 {
                        continue;
                    }

                    q.push_back((new_h, new_w, d + 1));
                }

                // eprintln!(
                //     "{}, {}",
                //     q.is_empty(),
                //     is_visit.iter().all(|row| row.iter().all(|a| *a))
                // );
            }

            ans = max(ans, new_ans);
        }
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
