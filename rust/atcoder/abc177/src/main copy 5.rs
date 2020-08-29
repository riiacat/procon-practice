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

const MOD: i64 = 1000_000_000 + 7;

//abc177-D
// #[fastout]
fn main() {
    input![n: usize, m: i64, ab: [(usize, usize); m]];

    let mut adj = vec![HashSet::new(); n as usize];

    for (a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;

        adj[a].insert(b);
        adj[b].insert(a);
    }

    let adj: Vec<Vec<usize>> = adj
        .into_iter()
        .map(|set| set.into_iter().collect())
        .collect();

    let mut is_visits = vec![false; n];

    let mut ans: i64 = -1;
    for i in 0..n as usize {
        if is_visits[i] {
            continue;
        }

        ans = max(ans, go(0, i, &mut is_visits, &adj) as i64);
    }

    // eprintln!("{:?}", adj);
    println!("{}", ans);
}

fn go(mid_ans: usize, start: usize, is_visits: &mut Vec<bool>, adj: &Vec<Vec<usize>>) -> usize {
    is_visits[start] = true;

    let mut ans = 1;
    for to in adj[start].iter() {
        if is_visits[*to] {
        } else {
            ans += go(mid_ans, *to, is_visits, adj);
        }
    }

    return ans;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
