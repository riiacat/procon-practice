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
    static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
}

//abc068-D
// #[fastout]
fn main() {
    input![n: usize, mut ass:  [usize; n]];

    ass.sort();
    let mut ans = ass.len();

    let pivot = ass.len();

    let mut double_l = 0;
    let mut old = 1000000000;
    for i in 0..pivot {
        if old == ass[i] {
            double_l += 1;
        }
        old = ass[i];
    }

    let mut double_r = 0;
    let mut old = 1000000000;
    for i in pivot..ass.len() {
        if old == ass[i] {
            double_l += 1;
        }
        old = ass[i];
    }

    if (double_l <= double_r) {
        //make r
    }
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
