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

const MOD: i64 = 1000_000_000_000_000 + 7;

//abc177-D
// #[fastout]
fn main() {
    input![n: usize, a: [usize; n]];

    let mut product = 1.to_bigint().unwrap();
    for a in a.iter() {
        product *= a.to_bigint().unwrap();
    }

    let mut l = a[0].to_bigint().unwrap();
    for i in 1..a.len() {
        let a = a[i].to_bigint().unwrap();

        if l >= a {
            l = lcm(l, a);
        } else {
            l = lcm(a, l);
        }

        if l > product {
            break;
        }
    }

    if l == product {
        println!("pairwise coprime");
        return;
    }

    // todo gcd of all.
    let mut l = a[0];
    let mut aa = Zero::zero();
    for i in 1..a.len() {
        aa = a[i];

        if l >= aa {
            l = gcdusize(l, aa);
        } else {
            l = gcdusize(aa, l);
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

fn lcm(l: BigInt, r: BigInt) -> BigInt {
    let p = &l * &r;
    return p / gcd(l, r);
}

fn gcd(a: BigInt, b: BigInt) -> BigInt {
    if b == Zero::zero() {
        return a;
    } else {
        let md = a % &b;
        return gcd(b, md);
    }
}

fn gcdusize<'a>(a: usize, b: usize) -> usize {
    if b == Zero::zero() {
        return a;
    } else {
        return gcdusize(b, a % b);
    }
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
