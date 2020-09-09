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
use itertools::concat;
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

//abc147-C
// #[fastout]
fn main() {
    input![n: usize];

    let mut a = Vec::new();
    let mut xy: Vec<Vec<_>> = Vec::new();

    for _ in 0..n {
        input![ai: usize, xyi: [(usize, usize); ai]];
        a.push(ai);
        xy.push(xyi.iter().map(|(p, is)| (*p - 1, *is)).collect());
    }

    let mut ans = -1;
    for i in 0..2usize.pow((n + 1) as u32) {
        let mut mid = 0;
        let mut is_valid = true;
        for j in 0..n {
            if (i >> j) & 1 == 0 {
                continue;
            }

            mid += 1;

            for (p, is_h) in xy[j].iter() {
                if (i >> p) & 1 != *is_h {
                    is_valid = false;
                }
            }
        }

        // eprintln!("{}, {}, {}, {}", is_valid, i, mid, 2 ^ (n + 1));
        if is_valid {
            ans = max(ans, mid);
        }
    }

    // eprintln!("{:?}", bit_counts);

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
