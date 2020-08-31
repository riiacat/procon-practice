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
    static ref XY: Mutex<Vec<Vec<Option<usize>>>> = Mutex::default();
}

// const MOD: usize = 9997063;
const MOD: usize = 100_000_000_ + 7;

//abc145-D
// #[fastout]
fn main() {
    input![x: usize, y: usize];

    {
        let mut xy = XY.lock().unwrap();
        *xy = vec![vec![None; y + 1]; x + 1];
    }

    // let mut ans = 0;

    println!("{}", go((0, 0), x as i64, y as i64));
}

fn go(start: (i64, i64), x: i64, y: i64) -> usize {
    // eprint!("{:?}, ", start);
    let (start_x, start_y) = start;

    if start_x == x && start_y == y {
        return 1;
    }

    {
        let x = x - start_x;
        let y = y - start_y;
        if (x + y) % 3 != 0 {
            return 0;
        }
    }

    {
        let xy = XY.lock().unwrap();
        // *xy = vec![vec![None; x + 1]; y + 1];
        if let Some(v) = xy[start_x as usize][start_y as usize] {
            return v;
        }
    }

    let new_x = start_x + 1;
    let new_y = start_y + 2;

    let mid_1 = if new_x > x || new_y > y {
        0
    } else {
        go((new_x, new_y), x, y)
    };

    let new_x = start_x + 2;
    let new_y = start_y + 1;

    let mid_2 = if new_x > x || new_y > y {
        0
    } else {
        go((new_x, new_y), x, y)
    };

    let a = (mid_1 + mid_2) % MOD;

    {
        let mut xy = XY.lock().unwrap();
        // *xy = vec![vec![None; x + 1]; y + 1];
        xy[start_x as usize][start_y as usize] = Some(a);
    }

    return a;
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
