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

//abc177-A
// #[fastout]
fn main() {
    input![d: i64, t: i64, s: i64];

    if d / s < t || (d / s == t && d % s == 0) {
        println!("Yes")
    } else {
        println!("No")
    }

    // println!("{}", ans);
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
