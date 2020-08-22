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
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::*;
use std::ops::Range;
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

//abc166-E
// #[fastout]
fn main() {
    input![n: usize, a: [i64; n]];

    let mut l = HashMap::new();
    let mut r = HashMap::new();

    for i in 0..n {
        let la = a[i] + (i + 1) as i64;
        if let Some(v) = l.get_mut(&la) {
            *v += 1;
        } else {
            l.insert(la, 1 as u64);
        }

        let ra = (i + 1) as i64 - a[i];
        if let Some(v) = r.get_mut(&ra) {
            *v += 1;
        } else {
            r.insert(ra, 1 as u64);
        }
    }

    let mut ans = 0;
    for (la, num) in l.iter() {
        if let Some(num_r) = r.get(la) {
            ans += num * num_r;
        } else {
        }
    }

    println!("{}", ans);
    eprintln!("{}", log10(std::u64::MAX as f64));
}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
