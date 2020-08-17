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
use std::collections::{BinaryHeap, HashSet};
use std::io::*;
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

//abc175-C
// #[fastout]
fn main() {
    input![mut x: i64, k: i64, d: i64];

    let is_minus = x < 0;
    x = if is_minus { -x } else { x };

    // let x: BigInt = x.into();
    // let k: BigInt = k.into();
    // let d: BigInt = d.into();

    //中に入ることはできる
    let count_inner = &x / &d;
    if count_inner >= k {
        // eprintln!("count_inner: {}", count_inner);
        println!("{}", x - k * d);
        return;
    }

    let remaind = x % d;
    let remaind_inv = d - remaind;
    let rest = k - count_inner;
    if (rest % 2 == 0) {
        println!("{}", remaind);
    } else {
        println!("{}", remaind_inv);
    }
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
