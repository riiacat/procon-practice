// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_traits::Pow;
use proconio::marker::Chars;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use itertools::concat;
// use itertools_num::*;
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
    input![n: usize, k: usize, mut s: Chars];
    let n = n;
    let s = s;

    let mut old_is_inv = true;
    let mut len = 0;
    let mut inv_lens = Vec::new();
    let mut stand_lens = Vec::new();

    // if *s.first().unwrap() == '0' {
    //     inv_lens.push((true, 0));
    // }

    for s in s.iter() {
        let is_inv = *s == '1';

        if (is_inv == old_is_inv) {
            len += 1;
        } else if old_is_inv {
            inv_lens.push((old_is_inv, len));
            len = 1;
        } else {
            stand_lens.push((old_is_inv, len));
            len = 1;
        }

        old_is_inv = is_inv;
    }

    if *s.last().unwrap() == '1' {
        inv_lens.push((true, len));
        stand_lens.push((false, 0));
    } else {
        stand_lens.push((false, len));
    }

    // eprintln!("{:?}", inv_lens);
    // eprintln!("{:?}", stand_lens);

    let inv_comsum = inv_lens.iter().fold(Vec::new(), |mut mid, (_, new)| {
        mid.push((mid.last().unwrap_or(&0) + *new) as i64);
        mid
    });
    let stand_comsum = stand_lens.iter().fold(Vec::new(), |mut mid, (_, new)| {
        mid.push((mid.last().unwrap_or(&0) + *new) as i64);
        mid
    });

    // eprintln!("{:?}", inv_comsum);
    // eprintln!("{:?}", stand_comsum);

    let mut ans = -1;
    for i in 0..inv_comsum.len() {
        let r = min(i + k, inv_comsum.len() - 1);
        let i_stand = min(i, inv_comsum.len() - 1);
        let r_stand = if r == 0 {
            0
        } else {
            min(r - 1, inv_comsum.len() - 1)
        };
        // eprintln!(
        //     "{}, {}, {}, {}, {}, {}, {}",
        //     i,
        //     r,
        //     stand_comsum[r_stand],
        //     inv_comsum[r],
        //     if i == 0 { 0 } else { stand_comsum[i_stand - 1] },
        //     if i == 0 { 0 } else { inv_comsum[i - 1] },
        //     stand_comsum[r_stand] + inv_comsum[r]
        //         - if i == 0 { 0 } else { stand_comsum[i_stand - 1] }
        //         - if i == 0 { 0 } else { inv_comsum[i - 1] }
        // );

        ans = max(
            ans,
            stand_comsum[r_stand] + inv_comsum[r]
                - if i == 0 { 0 } else { stand_comsum[i_stand - 1] }
                - if i == 0 { 0 } else { inv_comsum[i - 1] },
        );
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
