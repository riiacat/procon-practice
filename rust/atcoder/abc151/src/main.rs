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
use libm::*;
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

//abc147-C
// #[fastout]
fn main() {
    input![n: usize];

    let mut a_nums = Vec::new();
    let mut a_s = Vec::new();
    for _ in 0..n {
        input![a_num: usize, a: [(usize, usize); a_num]];
        a_nums.push(a_num);
        a_s.push(a);
    }

    let a_s = a_s;

    let mut ans = -1;
    for i in 0..2 ^ (n + 1) {
        let mut clues = vec![None; n];
        let mut is_valid = true;

        let mut flag = i;
        let mut new_ans = 0;
        let mut man_idx = 0;
        while flag != 0 {
            let valid_clues 
            if flag % 2 != 0 {
                new_ans += 1;

                
            }

            flag /= 2;
            man_idx += 1;
        }

        if is_valid {
            eprintln!("flag:{:#019b},  {}", i, new_ans);
            eprintln!("{:?}", clues);
        }

        if is_valid {
            ans = max(ans, new_ans);
        }
    }

    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
