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
use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::*;
use std::ops::Range;
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

//abc175-E
// #[fastout]
fn main() {
    input![
        r: usize,
        c: usize,
        k: usize,
        cell: [(usize, usize, usize); k]
    ];

    // eprintln!("{}", r);
    // eprintln!("{}", r);
    // eprintln!("{}", k);
    // eprintln!("{:?}", cell);

    // let mut cs = vec![Vec::new(); r];
    // let mut vs = vec![Vec::new(); r];
    let mut items = vec![vec![None; c]; r];
    // let mut com_count = vec![vec![0; c]; r];

    for (ri, ci, vi) in cell.iter() {
        // cs[ri - 1].push(ci - 1);
        // vs[ri - 1].push(vi - 1);
        items[ri - 1][ci - 1] = Some(*vi);
    }

    {
        let a = cell;
    }

    // for i in 0..r {
    //     cs[i].sort();
    //     vs[i].sort();
    // }

    // for ri in 0..r {
    //     for ci in 0..c {
    //         if let Some(_) = items.get(&(ri, ci)) {
    //             com_count[ri][ci] = if ci == 0 || ci == c - 1 {
    //                 1
    //             } else {
    //                 com_count[ri][ci - 1] + 1
    //             }
    //         }
    //     }
    // }

    let mut anss: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; 4]; c]; r];

    let goal_item_v = items[r - 1][c - 1].unwrap_or(0);
    anss[r - 1][c - 1][0] = 0 + goal_item_v as u64;
    anss[r - 1][c - 1][1] = 0 + goal_item_v as u64;
    anss[r - 1][c - 1][2] = 0 + goal_item_v as u64;
    anss[r - 1][c - 1][3] = 0;

    for delta_c in 0..c - 1 {
        let delta_c = (c - 2) - delta_c;
        for a in 0..3 {
            anss[r - 1][delta_c][a] = if let Some(v) = items[r - 1][delta_c] {
                max(
                    anss[r - 1][delta_c + 1][a + 1] + v as u64,
                    anss[r - 1][delta_c + 1][a],
                )
            } else {
                anss[r - 1][delta_c + 1][a]
            };
        }

        anss[r - 1][delta_c][3] = anss[r - 1][delta_c + 1][3];

        // eprintln!("[{},{}][0] = {}", r - 1, delta_c, anss[r - 1][delta_c][0]);
        // eprintln!("[{},{}][1] = {}", r - 1, delta_c, anss[r - 1][delta_c][1]);
        // eprintln!("[{},{}][2] = {}", r - 1, delta_c, anss[r - 1][delta_c][2]);
        // eprintln!("[{},{}][3] = {}", r - 1, delta_c, anss[r - 1][delta_c][3]);
    }
    for delta_r in 0..r - 1 {
        let delta_r = r - 2 - delta_r;
        for a in 0..4 {
            anss[delta_r][c - 1][a] = if let Some(v) = items[delta_r][c - 1] {
                anss[delta_r + 1][c - 1][0] + v as u64
            } else {
                anss[delta_r + 1][c - 1][0]
            }
        }

        // eprintln!("[{},{}][0] = {}", delta_r, c - 1, anss[delta_r][c - 1][0]);
        // eprintln!("[{},{}][1] = {}", delta_r, c - 1, anss[delta_r][c - 1][1]);
        // eprintln!("[{},{}][2] = {}", delta_r, c - 1, anss[delta_r][c - 1][2]);
        // eprintln!("[{},{}][3] = {}", delta_r, c - 1, anss[delta_r][c - 1][3]);
    }

    // eprintln!("r,c = {},{}", r, c);
    if r == 1 || c == 1 {
        // println!("{:?}", anss);
        println!("{}", anss[0][0][0]);
        return;
    }

    for l_s in 1..max(r, c) {
        let r_idx = max((r - 1) as i64 - l_s as i64, 0) as usize;
        let c_idx = max((c - 1) as i64 - l_s as i64, 0) as usize;
        // eprintln!("r_idx: {}, c_idx: {}", r_idx, c_idx);

        for delta_c in (c_idx + 1)..(c - 1) {
            let delta_c = (c_idx + 1) + (c - 2) - delta_c;
            eprintln!("(r_idx, delta_c)={}, {}", r_idx, delta_c);
            anss[r_idx][delta_c][3] = anss[min(r_idx + 1, r - 1)][delta_c][0];
            //delta_c ~ cまでにあとどれくらいアイテムがあるか？
            if let Some(v) = items[r_idx][delta_c] {
                for item_count in 0..3 {
                    anss[r_idx][delta_c][item_count] = *[
                        anss[r_idx][delta_c + 1][item_count],
                        anss[r_idx][delta_c + 1][item_count + 1] + v as u64,
                        anss[min(r_idx + 1, r - 1)][delta_c][0] + v as u64,
                    ]
                    .iter()
                    .max()
                    .unwrap();
                }
            } else {
                for item_count in 0..3 {
                    anss[r_idx][delta_c][item_count] = *[
                        anss[r_idx][delta_c + 1][item_count],
                        anss[min(r_idx + 1, r - 1)][delta_c][0],
                    ]
                    .iter()
                    .max()
                    .unwrap();
                }
            }

            // eprintln!("[{},{}][0] = {}", r_idx, delta_c, anss[r_idx][delta_c][0]);
            // eprintln!("[{},{}][1] = {}", r_idx, delta_c, anss[r_idx][delta_c][1]);
            // eprintln!("[{},{}][2] = {}", r_idx, delta_c, anss[r_idx][delta_c][2]);
            // eprintln!("[{},{}][3] = {}", r_idx, delta_c, anss[r_idx][delta_c][3]);
        }

        for delta_r in r_idx..r - 1 {
            let delta_r = r_idx + r - 2 - delta_r;
            // eprintln!("(delta_r, c_idx)={}, {}", delta_r, c_idx);

            anss[delta_r][c_idx][3] = anss[delta_r + 1][c_idx][0];
            //delta_c ~ cまでにあとどれくらいアイテムがあるか？
            if let Some(v) = items[delta_r][c_idx] {
                for item_count in 0..3 {
                    anss[delta_r][c_idx][item_count] = *[
                        anss[delta_r][c_idx + 1][item_count],
                        anss[delta_r][c_idx + 1][item_count + 1] + v as u64,
                        anss[min(r_idx + 1, r - 1)][c_idx][0] + v as u64,
                    ]
                    .iter()
                    .max()
                    .unwrap();
                }
            } else {
                for item_count in 0..3 {
                    anss[delta_r][c_idx][item_count] = *[
                        anss[delta_r][c_idx + 1][item_count],
                        anss[min(r_idx + 1, r - 1)][c_idx][0],
                    ]
                    .iter()
                    .max()
                    .unwrap();
                }
            }

            // eprintln!("[{},{}][0] = {}", delta_r, c_idx, anss[delta_r][c_idx][0]);
            // eprintln!("[{},{}][1] = {}", delta_r, c_idx, anss[delta_r][c_idx][1]);
            // eprintln!("[{},{}][2] = {}", delta_r, c_idx, anss[delta_r][c_idx][2]);
            // eprintln!("[{},{}][3] = {}", delta_r, c_idx, anss[delta_r][c_idx][3]);
        }
    }

    println!("{}", anss[0][0][0]);
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
