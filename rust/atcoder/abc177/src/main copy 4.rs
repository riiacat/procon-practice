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
// use ascii::Chars;
use libm::*;
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
    static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
}

const MOD: i64 = 1000_000_000 + 7;

//abc177-C
// #[fastout]
fn main() {
    input![n: i64, a: [i64; n]];

    let mut comsum = vec![0; n as usize];
    // comsum[0] = a[0];

    let mut old = 0;
    for i in 0..a.len() {
        comsum[i] = (a[i] + old) % MOD;
        old = comsum[i];
    }

    let mut ans = 0;
    for i in 0..a.len() - 1 {
        let m = a[i] * ((comsum[a.len() - 1] - comsum[i] + MOD) % MOD) % MOD;
        ans = (ans + m) % MOD;
    }

    println!("{}", ans);
}

// fn go2(start: usize, plus_val: usize, anss: &mut Vec<usize>, dcc: &Vec<usize>) {
//     let childrens = CHILDS.lock().unwrap()[start].clone();

//     // let mut ans = plus_val;
//     for c in childrens.iter() {
//         go2(*c, plus_val + dcc[start], anss, dcc);
//     }

//     anss[start] = plus_val + dcc[start];
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
