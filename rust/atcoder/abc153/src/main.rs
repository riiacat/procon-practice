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

//abc153-E
// #[fastout]
fn main() {
    input![h: i64, n: usize, ab: [(i64, i64); n]];

    let a: Vec<_> = ab.iter().map(|(a, _)| a).collect();
    let b: Vec<_> = ab.iter().map(|(_, b)| b).collect();

    let mut dp = vec![std::usize::MAX; h as usize + 1];

    for hh in 0..(h + 1) as usize {
        if hh == 0 {
            dp[hh] = 0;
        }

        for m_idx in 0..n as usize {
            dp[hh] = min(
                dp[hh],
                dp[max(hh as i64 - a[m_idx], 0) as usize] + *b[m_idx] as usize,
            );
        }
    }

    println!("{}", dp[h as usize]);
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
