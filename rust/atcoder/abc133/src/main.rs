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
    static ref H: Mutex<Vec<i32>> = Mutex::default();
    static ref W: Mutex<Vec<i32>> = Mutex::default();
}

//abc163-D
#[fastout]
fn main() {
    input![n: usize, a: [usize; n]];

    let a_sum_div2: usize = a.iter().sum::<usize>() / 2;

    let mut idx = 2;
    let mut a1 = a_sum_div2;
    loop {
        if idx - 1 >= a.len() {
            break;
        }

        a1 -= a[idx - 1];
        idx += 2;
    }

    let mut anss = vec![0; n];
    anss[0] = a1;
    // print!("{} ", a1);
    for i in 1..n {
        anss[i] = a[i - 1] - anss[i - 1];
    }

    // let ans = anss.iter().fold(String::new(), |mut mid_res, item| {
    //     mid_res.push_str(&format!("{} ", item * 2));
    //     mid_res
    // });

    for i in anss.iter() {
        print!("{} ", i * 2);
    }

    // println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
