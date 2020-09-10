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
use proconio::marker::Chars;
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
    input![h: usize, w: usize, s: [Chars; h]];

    // let s: Vec<_> = s
    //     .into_iter()
    //     .map(|s: String| AsciiString::from_str(&s[..]).unwrap())
    //     .collect();

    // let mut maze = vec![vec!['#'; w + 2]; h + 2];
    // eprintln!("before init");

    // for hh in 1..(h + 1) {
    //     for ww in 1..(w + 1) {
    //         maze[hh][ww] = s[hh - 1][ww - 1];
    //         if maze[hh][ww] == '#' {
    //             // is_visit[hh - 1][ww - 1] = true;
    //         }
    //     }
    // }

    // eprintln!("after init");
    let del_h = [-1, 1, 0, 0];
    let del_w = [0, 0, -1, 1];
    let mut is_visit = vec![false; w * h];
    let mut ans = -1;

    // let a = is_visit[0][1];

    for hh in 0..h {
        for ww in 0..w {
            if s[hh][ww] == '#' {
                continue;
            }

            {
                for hh in 0..h {
                    for ww in 0..w {
                        is_visit[hh * w + ww] = false;
                    }
                }
            }

            // let mut is_visit: Vec<_> = is_visit.iter().map(|v| v.clone()).collect();
            // let mut dist = vec![vec![1000; w]; h];
            let mut new_ans = -1;
            //hh, wwをスタートにして、幅優先探索をする
            let mut q = VecDeque::new();
            q.clear();
            q.push_back((hh, ww, 0));
            is_visit[hh * w + ww] = true;
            while let Some((hh, ww, d)) = q.pop_front() {
                let mut is_comp = true;
                {
                    // for hh in 0..h {
                    //     for ww in 0..w {
                    //         if !is_visit[hh * w + ww] && s[hh][ww] != '#' {
                    //             is_comp = false;
                    //         }
                    //     }
                    // }
                    // if is_comp {
                    //     break;
                    // }
                }
                // is_visit[hh * w + ww] = true;

                // while !(q.is_empty()) {
                // let ;
                // dist[hh - 1][ww - 1] = d;
                // eprintln!("{:?}", ((hh, ww), d));
                new_ans = max(new_ans, d);

                for i in 0..4 {
                    let new_h = hh as i64 + del_h[i];
                    let new_w = ww as i64 + del_w[i];

                    if new_h < 0 || new_w < 0 || new_h >= h as i64 || new_w >= w as i64 {
                        continue;
                    }

                    let new_h = new_h as usize;
                    let new_w = new_w as usize;

                    // eprintln!("{}, {}, {}", new_h, new_w, maze[new_h][new_w]);
                    // if maze[new_h][new_w] == '#' || dist[new_h - 1][new_w - 1] < 1000 {
                    if s[new_h][new_w] == '#' || is_visit[new_h * w + new_w] {
                        continue;
                    }

                    q.push_back((new_h, new_w, d + 1));
                    is_visit[new_h * w + new_w] = true;
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
