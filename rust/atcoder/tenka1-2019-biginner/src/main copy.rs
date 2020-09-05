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

//tenka1-2019-C
// #[fastout]
fn main() {
    input![n: usize, s: Chars];

    let mut continuas_region = Vec::new();

    let mut old_is_w = true;
    let mut len = 0;
    for s in s.iter() {
        if *s == '.' && old_is_w {
            len += 1;
            eprintln!("white len: {}", len);
        } else if *s == '#' && !old_is_w {
            len += 1;
            eprintln!("black len: {}", len);
        } else {
            eprintln!("switch len: {}", len);
            continuas_region.push((old_is_w, len));
            len = 1;
        }

        old_is_w = *s == '.';
    }

    if *s.last().unwrap() == '.' {
        continuas_region.push((true, len));
    }

    eprintln!("{:?}", continuas_region);

    let mut old = (true, 100000000);
    let mut ans = 0;
    for (is_w, len) in continuas_region.iter() {
        let (old_is_w, old_len) = old;
        if *is_w && !old_is_w {
            ans += min(*len, old_len);
        }

        old = (*is_w, *len);
    }

    let mut ans2 = 0;
    for (is_w, len) in continuas_region.iter().rev() {
        let (old_is_w, old_len) = old;
        if *is_w && !old_is_w {
            ans += min(*len, old_len);
        }

        old = (*is_w, *len);
    }

    // eprintln!("{:?}", adj);
    println!("{}", ans);
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
