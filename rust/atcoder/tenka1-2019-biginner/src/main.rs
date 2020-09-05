// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::Pow;
use num_traits::{One, Zero};

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
    // static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    // static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
    static ref ADJ: Mutex<Vec<Vec<usize>>> = Mutex::default();
}

// const MOD: usize = 9997063;
// const MOD: usize = 100_000_000_ + 7;

//abc177-D
// #[fastout]
fn main() {
    input![n: usize, a: [usize; n]];

    // let a: u
    let mut product = 1;
    for a in a.iter() {
        product *= a;
        // product %= MOD;
    }

    let mut l = a[0];
    for i in 1..a.len() {
        let a = a[i];

        if l >= a {
            // l = lcm(l, a) % MOD;
        } else {
            // l = lcm(a, l) % MOD;
        }

        // if l > product {
        //     break;
        // }
    }

    if l == product {
        println!("pairwise coprime");
        return;
    }

    // todo gcd of all.
    let mut l = a[0] as u128;
    let mut aa = Zero::zero();
    for i in 1..a.len() {
        aa = a[i] as u128;

        if l >= aa {
            l = gcdu128(l as u128, aa as u128);
        } else {
            l = gcdu128(aa, l);
        }

        if l == One::one() {
            println!("setwise coprime");
            return;
        }
    }

    if l == One::one() {
        println!("setwise coprime");
        return;
    }

    println!("not coprime");

    // eprintln!("{:?}", adj);
    // println!("{}", ans);
}

fn lcm(l: u128, r: u128) -> u128 {
    let p = &l * &r;
    return p / gcd(l, r);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == Zero::zero() {
        return a;
    } else {
        let md = a % &b;
        return gcd(b, md);
    }
}

fn gcdu128<'a>(a: u128, b: u128) -> u128 {
    if b == Zero::zero() {
        return a;
    } else {
        return gcdu128(b, a % b);
    }
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
