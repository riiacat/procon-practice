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

use itertools::Itertools;
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

//abc085-D
// #[fastout]
fn main() {
    input![n: usize, mut h: i64, ab: [(usize, usize); n]];

    let a_max = ab.iter().map(|(a, b)| *a).max().unwrap();
    let mut b = ab.iter().map(|(a, b)| *b).collect_vec();
    b.sort_by_key(|b| Reverse(*b));
    // eprintln!("{:?}", b);

    let mut ans: i64 = 0;
    for b in b.iter() {
        if a_max > *b {
            break;
        }

        if h <= 0 {
            println!("{}", ans);
            return;
        }

        h -= *b as i64;
        ans += 1;
    }

    while h > 0 {
        ans += 1;
        h -= a_max as i64;
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
