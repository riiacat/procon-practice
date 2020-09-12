// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::{abs, pow, One, Pow, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
// use ascii::Chars;
// use libm::*;
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

//abc112-D
// #[fastout]
fn main() {
    input![n: usize, m: usize];

    let mut ans = -1 as i64;
    for i in 1..((m as f64).sqrt() as usize + 1) {
        if m % i == 0 {
            let inv_i = m / i;
            if inv_i >= n {
                ans = max(ans, i as i64);
            }

            if i >= n {
                ans = max(ans, inv_i as i64);
            }
        }
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
