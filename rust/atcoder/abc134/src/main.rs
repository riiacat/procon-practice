// // -*- coding:utf-8-unix -*-

// #[macro_use]
// extern crate lazy_static;
// extern crate num_bigint; // 0.2.2
// extern crate num_traits; // 0.2.8
// use num_bigint::BigInt;
// use num_traits::Pow;
// use petgraph::algo::dijkstra;
// use petgraph::Directed;

// // use proconio::derive_readable;
// use proconio::fastout;
// use proconio::input;
// // use std::convert::TryInto;
// use ascii::{AsciiChar, AsciiString};
// use itertools::concat;
// use lazy_static::lazy_static;
// use libm::*;
// use petgraph::Graph;
// use std::cmp::*;
// use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
// use std::io::*;
// use std::ops::Range;
// use std::str::FromStr;
// use std::sync::Mutex;
// use superslice::*;

// pub fn read<T: FromStr>() -> T {
//     let stdin = stdin();
//     let stdin = stdin.lock();
//     let token: String = stdin
//         .bytes()
//         .map(|c| c.expect("failed to read char") as char)
//         .skip_while(|c| c.is_whitespace())
//         .take_while(|c| !c.is_whitespace())
//         .collect();
//     token.parse().ok().expect("failed to parse token")
// }

// // lazy_static! {
// //     static ref H: Mutex<Vec<i32>> = Mutex::default();
// //     static ref W: Mutex<Vec<i32>> = Mutex::default();
// // }

// // let mut values = VALUES.lock().unwrap();
// // values.extend_from_slice(&[1, 2, 3, 4]);
// // assert_eq!(&*values, &[1, 2, 3, 4]);

use proconio::{input, marker::Chars};
use std::collections::HashMap;

// static ASCII_LOWER: [char; 26] = [
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];

// abc172-D
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut b = vec![false; n];
    for i in (1..n + 1).into_iter().rev() {
        let mut num = i;
        let mut sum_b: usize = 0;
        while num <= n {
            sum_b += if b[num - 1] { 1 } else { 0 };
            num += i;
        }

        // eprintln!("{}, {}", i, sum_b);
        b[i - 1] = (sum_b % 2) != a[i - 1];
        //
        // if i * 2 > n {
        //     b[i - 1] = a[i - 1] == 1;
        // } else {
        //     b[i - 1] = a[2 * i - 1] != a[i - 1];
        // }
    }

    println!(
        "{}",
        b.iter()
            .map(|is_b| if *is_b { 1 } else { 0 })
            .sum::<usize>()
    );
    for (idx, b) in b.iter().enumerate() {
        if *b {
            println!("{}", idx + 1);
        }
    }
}
