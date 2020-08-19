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

//abc084-D
// #[fastout]
fn main() {
    input![q: usize, lrs: [(i64, i64); q]];

    let mut primes = Vec::new();
    primes.push(2);
    let mut comsum_like2017s: Vec<usize> = vec![0; 100001];
    // let mut comsum_like2017s = vec![0; 100];

    // for i in 3..(100) {
    for i in 3..(100000 + 1) {
        let mut is_prime = true;

        if i % 2 == 0 {
            is_prime = false;
        } else {
            for prime in primes.iter() {
                if *prime > (sqrt(i as f64) as usize) {
                    break;
                }
                if i % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
        }
        if is_prime {
            primes.push(i);
        }

        let is_contain_div2 = primes.binary_search(&((i + 1) / 2)).is_ok();

        if is_prime && is_contain_div2 {
            comsum_like2017s[i] = comsum_like2017s[i - 2] + 1;
        } else {
            comsum_like2017s[i] = comsum_like2017s[i - 2];
        }
    }

    // let mut v: Vec<_> = primes.iter().collect();
    // v.sort();
    // eprintln!("{:?}", v);
    // let a = comsum_like2017s.as_slice();
    // eprintln!("{:?}", (&a[..100]).iter().enumerate().collect::<Vec<_>>());

    // println!("{}", maxes.iter().max().unwrap());

    for (l, r) in lrs.iter() {
        println!(
            "{}",
            comsum_like2017s[*r as usize] - comsum_like2017s[max(l - 2, 0) as usize]
        );
    }
}

// fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
