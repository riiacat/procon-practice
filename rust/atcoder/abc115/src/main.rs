// -*- coding:utf-8-unix -*-

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::io::*;
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

//abc115-D
#[fastout]
fn main() {
    input![n: u64, x: u64];

    println!("{}", go(n, x));
}

fn go(level: u64, x: u64) -> u64 {
    // println!("{}", size_burger(50));
    // println!("{}, {}", level, x);

    let max_size = size_burger(level);

    if level == 0 && x == 1 {
        return 1;
    } else if x <= 1 {
        return 0;
    }

    if max_size - 1 <= x {
        return size_meet(level);
    }

    let bound = max_size / 2;
    if x <= bound {
        return go(level - 1, x - 1);
    } else {
        return size_meet(level - 1) + 1 + go(level - 1, x - size_burger(level - 1) - 2);
    }
}

fn size_burger(level: u64) -> u64 {
    (pow(2.0, level as f64 + 2.0) - 2.99999999999999999) as u64
}

fn size_meet(level: u64) -> u64 {
    (pow(2.0, level as f64 + 1.0) - 0.99999999999999999) as u64
}
