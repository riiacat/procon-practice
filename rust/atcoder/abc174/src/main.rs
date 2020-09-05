// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::abs;
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

//abc174-E
// #[fastout]
fn main() {
    input![n: usize, k: usize, a: [usize; n]];

    let mut ng = -100;
    let mut ok = *a.iter().max().unwrap() as i64;

    /* ok と ng のどちらが大きいかわからないことを考慮 */
    while abs(ok - ng) > 1 {
        // eprintln!("{}, {}", ng, ok);
        let mid = (ok + ng) / 2;

        if is_ok(mid, k, &a) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    // return ok;

    println!("{}", ok);
}

fn is_ok(v: i64, k: usize, a: &Vec<usize>) -> bool {
    if v <= 0 {
        return false;
    }
    return a
        .iter()
        .map(|a| (*a as f64 / v as f64).ceil() as i64 - 1)
        .sum::<i64>()
        <= k as i64;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
