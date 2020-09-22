// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::{abs, pow, One, Pow, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
// use ascii::Chars;
// use libm::*;
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
    // static ref IS_VISITS: Mutex<Vec<bool>> = Mutex::default();
    // static ref TO_SETS: Mutex<Vec<Option<&HashSet<usize>>>> = Mutex::default();
    // static ref IS_VISITS: Mutex<Vec<bool>>> = Mutex::default();
    static ref ADJ_LIKE: Mutex<Vec<Vec<usize>>> = Mutex::default();

}

// const MOD: usize = 9997063;
// const MOD: usize = 100_000_000_ + 7;

//abc157-D
// #[fastout]
fn main() {
    input![
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k]
    ];

    let mut adjs_like = ADJ_LIKE.lock().unwrap();
    *adjs_like = vec![Vec::new(); n];

    for (a, b) in ab.iter() {
        let (a, b) = (a - 1, b - 1);
        adjs_like[a].push(b);
        adjs_like[b].push(a);
    }

    let mut dislikes = vec![HashSet::new(); n];
    for (c, d) in cd.iter() {
        let (c, d) = (c - 1, d - 1);
        dislikes[c].insert(d);
        dislikes[d].insert(c);
    }

    {
        // let mut is_visits = IS_VISITS.lock().unwrap();
        // *is_visits = vec![false; n];
    }
    let mut to_sets: Vec<Option<usize>> = vec![None; n];

    // let a = Some(1);

    let mut sets = Vec::new();

    let mut set_idx = 0;
    for i in 0..n {
        if to_sets[i].is_some() {
            continue;
        }

        let new_set = HashSet::new();
        let new_set = go(i, &mut to_sets, new_set, set_idx, &adjs_like);

        sets.push(new_set);
        set_idx += 1;
    }

    // eprintln!("{:?}", sets);
    // eprintln!("{:?}", dislikes);

    for i in 0..n {
        let set_num = to_sets[i].unwrap();
        let mut ans = sets[set_num].len();

        for dislikes in dislikes[i].iter() {
            if to_sets[*dislikes].unwrap() == set_num {
                // eprintln!("{}, {}", i, dislikes);
                ans -= 1;
            }
        }

        // eprintln!(
        //     "sets: {}, direct: {}",
        //     sets[set_num].len() - 1,
        //     adjs_like[i].len()
        // );
        ans -= adjs_like[i].len();
        // eprintln!("{} ", ans - 1);
        // println!("{} ", ans - 1);
        print!("{} ", ans - 1);
    }
}

fn go(
    idx: usize,
    to_sets: &mut Vec<Option<usize>>,
    mut set: HashSet<usize>,
    set_idx: usize,
    adjs_like: &Vec<Vec<usize>>,
) -> HashSet<usize> {
    // eprintln!("{}, {:?}", idx, to_sets);
    if to_sets[idx].is_some() {
        return set;
    }

    set.insert(idx);
    // {
    to_sets[idx] = Some(set_idx);
    let adj_like = &adjs_like[idx];

    let mut current_set = set;
    for to in adj_like.iter() {
        if to_sets[*to].is_some() {
            continue;
        }
        current_set = go(*to, to_sets, current_set, set_idx, adjs_like);
    }

    // }

    return current_set;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
