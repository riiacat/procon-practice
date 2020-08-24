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

// //abc110-C
// // #[fastout]
// fn main() {
//     input![s: String, t: String];

//     let s = AsciiString::from_str(&s[..]).unwrap();
//     let t = AsciiString::from_str(&t[..]).unwrap();

//     let mut s_map = HashMap::new();
//     let mut t_map = HashMap::new();

//     for s_char in s.chars() {
//         let e = s_map.entry(s_char).or_insert(0);
//         *e += 1;
//     }

//     for t_char in t.chars() {
//         let e = t_map.entry(t_char).or_insert(0);
//         *e += 1;
//     }

//     if s_map.keys().len() == 26 {
//         //sortして同じだったらOK
//         let mut s: Vec<_> = s.chars().collect();
//         let mut t: Vec<_> = t.chars().collect();
//         s.sort();
//         t.sort();
//         if s != t {
//             println!("No");
//         } else {
//             println!("Yes");
//         }
//         return;
//     }

//     // let mut s_items: Vec<_> = s_map.iter().collect();
//     // let mut t_items: Vec<_> = s_map.iter().collect();

//     let alphabets: HashSet<_> = AsciiString::from_str("abcdefghijklmnopqrstuvwxyz")
//         .unwrap()
//         .chars()
//         .collect();

//     let s_alphabets: HashSet<_> = s.chars().collect();

//     let not_used_alphabets: HashSet<_> = alphabets.difference(&s_alphabets).collect();

//     let mut s = s;

//     for i in 0..s.len() {
//         if s[i] == t[i] {
//             continue;
//         }

//         if not_used_alphabets.contains(&t[i]) {
//             s[i] = t[i];
//         }
//     }

//     let mut s: Vec<_> = s.chars().collect();
//     let mut t: Vec<_> = t.chars().collect();
//     s.sort();
//     t.sort();
//     if s != t {
//         println!("No");
//     } else {
//         println!("Yes");
//     }
//     return;

//     // println!("{}", ans);
// }

// // let mut values = VALUES.lock().unwrap();
// // values.extend_from_slice(&[1, 2, 3, 4]);
// // assert_eq!(&*values, &[1, 2, 3, 4]);

use proconio::{input, marker::Chars};
use std::collections::HashMap;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dict = HashMap::new();
    let mut dict_inv = HashMap::new();
    for (c_s, c_t) in s.into_iter().zip(t.into_iter()) {
        if let Some(&val) = dict.get(&c_s) {
            if val != c_t {
                println!("No");
                return;
            }
        } else {
            dict.insert(c_s, c_t);
        }
        if let Some(&val) = dict_inv.get(&c_t) {
            if val != c_s {
                println!("No");
                return;
            }
        } else {
            dict_inv.insert(c_t, c_s);
        }
    }
    println!("Yes");
}
