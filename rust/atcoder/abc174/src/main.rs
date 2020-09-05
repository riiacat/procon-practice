// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::Pow;
use num_traits::{One, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
// use ascii::Chars;
use proconio::marker::{Bytes, Chars};
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
    // static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    // static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
    static ref ADJ: Mutex<Vec<Vec<usize>>> = Mutex::default();
}

// const MOD: usize = 9997063;
// const MOD: usize = 100_000_000_ + 7;

//tenka1-2019-C
// #[fastout]
fn main() {
    input![n: usize, s: Chars];

    // let mut continuas_region = Vec::new();

    let mut b_count = vec![-1; n + 1];
    let mut w_count = vec![-1; n + 1];
    b_count[0] = 0;
    w_count[0] = 0;
    for (idx, s) in s.iter().enumerate() {
        let idx = idx + 1;
        b_count[idx] = b_count[idx - 1];
        w_count[idx] = w_count[idx - 1];
        if *s == '#' {
            b_count[idx] += 1;
        } else {
            w_count[idx] += 1;
        }
    }

    let all_w = w_count[n];
    for i in 0..n + 1 {
        w_count[i] = all_w - w_count[i];
    }

    // eprintln!("{:?}", b_count);
    // eprintln!("{:?}", w_count);

    let mut old_is_w = true;
    let mut ans = 1000000000;
    for (idx, s) in s.iter().enumerate() {
        let is_w = *s == '.';
        // if !old_is_w && is_w {
        //     eprintln!("{}, {}", idx, s);
        ans = min(ans, w_count[idx] + b_count[idx]);
        // }

        old_is_w = is_w;
    }

    ans = min(ans, w_count[n] + b_count[n]);

    // eprintln!("{:?}", adj);
    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
