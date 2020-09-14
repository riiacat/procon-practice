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
    static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
}

//abc138-D
// #[fastout]
fn main() {
    input![
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        query: [(usize, usize); q]
    ];

    {
        let mut parents = PARENTS.lock().unwrap();
        let mut childrens = CHILDS.lock().unwrap();

        parents.clear();
        childrens.clear();

        for _ in 0..n {
            childrens.push(Vec::new());
            parents.push(std::usize::MAX);
        }

        for (a, b) in ab.iter() {
            let a = a - 1;
            let b = b - 1;
            parents[b] = a;
            childrens[a].push(b);
        }
    }

    let mut deep_child_count = vec![0; n];

    // go(0, &mut deep_child_count);

    let mut ans = 0;
    for (p, x) in query.iter() {
        let p = p - 1;
        deep_child_count[p] += x;
    }

    let mut anss = vec![std::usize::MAX; n];

    go2(0, 0, &mut anss, &deep_child_count);

    for ans in anss.iter() {
        print!("{} ", ans);
    }
}

fn go2(start: usize, plus_val: usize, anss: &mut Vec<usize>, dcc: &Vec<usize>) {
    let childrens = CHILDS.lock().unwrap()[start].clone();

    // let mut ans = plus_val;
    for c in childrens.iter() {
        go2(*c, plus_val + dcc[start], anss, dcc);
    }

    anss[start] = plus_val + dcc[start];
}

fn go(start: usize, dcc: &mut Vec<usize>) {
    eprintln!("start: {}, dcc: {:?}", start, dcc);
    // let parents = PARENTS.lock().unwrap();
    // let mut childrens_2 =
    let childrens = CHILDS.lock().unwrap()[start].clone();

    let mut ans = 1;
    for c in childrens.iter() {
        go(*c, dcc);
        ans += dcc[*c];
    }

    dcc[start] = ans;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
