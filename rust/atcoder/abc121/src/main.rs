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
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

//abc121-D
// #[fastout]
fn main() {
    input![h: usize, w: usize, s: [String; h]];

    // let a: String = String::new("abc");
    // let b = ;

    let mut num_white_without_se = 0;
    for i in 0..w {
        for j in 0..h {
            if s[j].chars().nth(i).unwrap() == '.' {
                eprintln!("num_white_without_se: {}", num_white_without_se);
                num_white_without_se += 1;
            } else if s[j].chars().nth(i).unwrap() == '#' {
            }
        }
    }

    let mut q = VecDeque::new();

    let mut is_visits = vec![vec![false; w]; h];
    //h,w,depth
    q.push_back(((0, 0), 1));
    is_visits[0][0] = true;
    let mut target = None;

    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let (current_pos, d) = &current;

        if *current_pos == (h - 1, w - 1) {
            target = Some(current);
            break;
        }

        let (c_h, c_w) = *current_pos;
        if c_h > 0 && s[c_h - 1].chars().nth(c_w).unwrap() != '#' && !is_visits[c_h - 1][c_w] {
            q.push_back(((c_h - 1, c_w), *d + 1));
            is_visits[c_h - 1][c_w] = true;
        }
        if c_h < h - 1 && s[c_h + 1].chars().nth(c_w).unwrap() != '#' && !is_visits[c_h + 1][c_w] {
            q.push_back(((c_h + 1, c_w), *d + 1));
            is_visits[c_h + 1][c_w] = true;
        }

        if c_w > 0 && s[c_h].chars().nth(c_w - 1).unwrap() != '#' && !is_visits[c_h][c_w - 1] {
            q.push_back(((c_h, c_w - 1), *d + 1));
            is_visits[c_h][c_w - 1] = true;
        }
        if c_w < w - 1 && s[c_h].chars().nth(c_w + 1).unwrap() != '#' && !is_visits[c_h][c_w + 1] {
            q.push_back(((c_h, c_w + 1), *d + 1));
            is_visits[c_h][c_w + 1] = true;
        }
    }

    eprintln!("num_white_without_se: {}", num_white_without_se);
    let ans = target.map(|(_, d)| num_white_without_se - d).unwrap_or(-1);
    println!("{}", ans);
}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
