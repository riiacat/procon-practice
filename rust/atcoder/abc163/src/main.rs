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

//abc163-D
// #[fastout]
fn main() {
    input![n: usize, h: usize, ab: [(usize, usize); n]];

    let mut ab_sorted = Vec::new();
    for (a, b) in ab.iter() {
        ab_sorted.push(Reverse((true, *a)));
        ab_sorted.push(Reverse((false, *b)));
    }

    ab_sorted.sort();

    let mut comsum = Vec::new();
    comsum.push(Reverse((ab_sorted[0].0).1));
    for i in 1..(2 * n) {
        comsum.push(Reverse((ab_sorted[i].0).1 + comsum[i - 1].0));
    }

    let mut ans = std::i64::MAX;
    for (a, b) in ab.iter() {
        let mut mid_ans: i64 = 0;
        let mut h: i64 = h as i64;
        let idx = ab_sorted.upper_bound(&Reverse((true, *a)));
        eprintln!("{}, {:?}", a, ab_sorted);
        eprintln!("{:?}", comsum);
        eprintln!("{}", idx);

        mid_ans += idx as i64;
        h -= comsum[idx - 1].0 as i64;

        eprintln!("rest_h: {}, mid_ans: {}", h, mid_ans);
        if h > 0 {
            let a = *a as i64;
            mid_ans += h / a + if h % a != 0 { 1 } else { 0 };
        }
        eprintln!("mid_ans: {}", mid_ans);

        ans = min(mid_ans, ans);
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
