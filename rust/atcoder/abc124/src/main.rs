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

//abc147-D
// #[fastout]
fn main() {
    input![n: usize, mut a: [usize; n]];

    let mut bit_counts = Vec::new();

    for i in 0..60 {
        // eprintln!("{:?}", a);
        let mut count = 0;
        for a in a.iter_mut() {
            if *a & 1 == 1 {
                count += 1;
            }

            *a = *a >> 1;
        }

        bit_counts.push(count);
    }

    // eprintln!("{:?}", bit_counts);

    let mut ans = 0;
    let mut fac = 1;
    for (i, c) in bit_counts.iter().enumerate() {
        ans += (((fac * c) % MOD) * (n - c)) % MOD;
        ans %= MOD;

        fac *= 2;
        fac %= MOD;
    }
    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
