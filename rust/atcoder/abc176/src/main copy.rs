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

//abc176-A
// #[fastout]
fn main() {
    input![a: u128, b: u128];

    let mut ans: u128 = 0;
    for bit_num in 0..(log2(b as f64) as u128) + 1 {
        let a_remaind = a % (1 << (bit_num + 1)) + 1;
        let b_remaind = b % (1 << (bit_num + 1)) + 1;

        let mut is_1 = false;
        if bit_num == 0 && b - a <= 4 {
            is_1 = if a % 2 == 0 {
                match b - a {
                    0 => false,
                    1 => false,
                    2 => true,
                    3 => true,
                    4 => false,
                    _ => panic!("b-a 1"),
                }
            } else {
                match b - a {
                    0 => false,
                    1 => true,
                    2 => true,
                    3 => false,
                    4 => false,
                    _ => panic!("b-a 2"),
                }
            };
        } else {
            let a_1s = if a_remaind > (1 << bit_num) {
                (1 << (bit_num + 1)) - (a_remaind - 1)
            } else {
                0
            };

            let b_1s = if b_remaind > (1 << (bit_num)) {
                b_remaind - (1 << bit_num)
            } else {
                0
            };

            // eprintln!(
            //     "{}, a_r: {}, b_r: {}, a_1s: {}, b_1s:{}",
            //     bit_num, a_remaind, b_remaind, a_1s, b_1s
            // );
            is_1 = (a_1s + b_1s) % 2 == 1;
        }

        if is_1 {
            ans += 1 << bit_num;
        }
    }
    // eprintln!("{:#015b}", ans);
    println!("{}", ans);
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
