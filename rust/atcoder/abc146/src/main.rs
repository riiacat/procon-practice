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
    // static ref CHILDS: Mutex<Vec<Vec<usize>>> = Mutex::default();
    static ref ADJ: Mutex<Vec<Vec<usize>>> = Mutex::default();
    static ref PARENTS: Mutex<Vec<usize>> = Mutex::default();
    static ref TOIDX: Mutex<HashMap<(usize, usize), usize>> = Mutex::default();
}

// const MOD: usize = 9997063;
// const MOD: usize = 100_000_000_ + 7;
const LARGEN: usize = 10000000;
//abc146-D
// #[fastout]
fn main() {
    input![n: usize, ab: [(usize, usize); n - 1]];

    // let mut adjs = ADJ.lock().unwrap();
    let mut adjs = vec![Vec::new(); n];
    {
        let mut edgeToIdx = TOIDX.lock().unwrap();
        for (idx, (a, b)) in ab.iter().enumerate() {
            let a = a - 1;
            let b = b - 1;
            adjs[a].push(b);
            adjs[b].push(a);
            edgeToIdx.insert((a, b), idx);
        }
    }

    {
        let mut parents = PARENTS.lock().unwrap();
        *parents = vec![LARGEN; n];
        // eprintln!("{:?}", parents);
    }

    let mut is_visits = vec![false; n];
    let min_color = make_tree(0, &mut is_visits, &adjs);

    let mut colors = vec![10000000; n - 1];
    let mut is_visits = vec![false; n];

    let mut cant_use_color = None;
    make_color(0, cant_use_color, &mut colors, &mut is_visits, &adjs);

    {
        let parents = PARENTS.lock().unwrap();
        // eprintln!("{:?}", parents);
    }
    println!("{}", min_color);
    for c in colors.iter() {
        println!("{}", c + 1);
    }
    // eprintln!("{:?}", adj);
    // println!("{}", ans);
}

fn make_color(
    start: usize,
    cant_use_color: Option<usize>,
    colors: &mut Vec<usize>,
    is_visits: &mut Vec<bool>,
    adjs: &Vec<Vec<usize>>,
) {
    // eprintln!("make_color: {}", start);
    if is_visits[start] {
        return;
    }

    is_visits[start] = true;
    let mut is_root = true;
    // let adj = &ADJ.lock().unwrap()[start].clone();
    // let mut anss = Vec::new();

    let mut usable = 0;

    for to in adjs[start].iter() {
        if is_visits[*to] {
            {
                let mut parents = PARENTS.lock().unwrap();
                parents[start] = *to;
            }
            is_root = false;
            continue;
        }

        {
            let edgeToIdx = TOIDX.lock().unwrap();
            if let Some(cant_use) = cant_use_color {
                if cant_use == usable {
                    usable += 1;
                }
            }
            let edge_idx = *edgeToIdx.get(&(start, *to)).unwrap();
            colors[edge_idx] = usable;
            // eprintln!(
            //     "start: {}, to: {}, edge: {}, c: {}",
            //     start, to, edge_idx, usable
            // );
        }

        make_color(*to, Some(usable), colors, is_visits, adjs);
        usable += 1;
    }
}

// (myself, max)
fn make_tree(start: usize, is_visits: &mut Vec<bool>, adjs: &Vec<Vec<usize>>) -> usize {
    // if is_visits[start] {
    //     return ;
    // }

    is_visits[start] = true;
    let mut is_root = true;
    // let adj = &ADJ.lock().unwrap()[start].clone();
    let mut anss = Vec::new();
    for to in adjs[start].iter() {
        if is_visits[*to] {
            {
                let mut parents = PARENTS.lock().unwrap();
                parents[start] = *to;
            }
            is_root = false;
            continue;
        }

        anss.push(make_tree(*to, is_visits, adjs));
    }

    let ans = anss
        .iter()
        // .enumerate()
        // .map(|(min_color)| min_color)
        .max()
        .map(|min_color| max(*min_color, adjs[start].len()))
        .unwrap_or(1);

    // eprintln!("make_tree: {}, {}", start, ans);

    return ans;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
