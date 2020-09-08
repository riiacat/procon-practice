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
    static ref H: Mutex<Vec<i32>> = Mutex::default();
    static ref W: Mutex<Vec<i32>> = Mutex::default();
}

//abc176-D
// #[fastout]
fn main() {
    input![
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        dH: usize,
        dW: usize,
        s: [String; h]
    ];

    let ch = ch - 1;
    let cw = cw - 1;
    let dH = dH - 1;
    let dW = dW - 1;

    let mut dp = vec![vec![std::i32::MAX - 2; w]; h];
    dp[dH][dW] = 0;
    let mut is_visit = vec![vec![false; w]; h];

    // let ans = std::thread::Builder::new()
    //     .name("big stack size".into())
    //     .stack_size(32 * 1024 * 1024) // 32 MBのスタックサイズ
    //     // .stack_size(1024 * 100) // 32 MBのスタックサイズ
    //     .spawn(move || {
    //         // ここで深い再帰を実行
    //         go(ch, cw, &mut dp, &mut is_visit, h, w, &s, (10000, 10000))
    //     })
    //     .unwrap()
    //     .join()
    //     .unwrap();

    let ans = go(ch, cw, &mut dp, &mut is_visit, h, w, &s);

    // println!("{:?}", s);

    println!(
        "{}",
        if ans >= (std::i32::MAX - 1) as usize {
            -1
        } else {
            ans as i32
        }
    );
}


// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// -100000000
//  1000000000
