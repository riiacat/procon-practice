// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashSet};
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

//abc175-B
// #[fastout]
fn main() {
    input![n: usize, l: [u64; n]];
    // let set: HashSet<u64> = v.into_iter().collect();
    // let l: Vec<u64> = set.into_iter().collect();
    let size = l.len();

    let mut ans = 0;
    // eprintln!("{:?}", l);

    // let mut set = HashSet::new();

    for i in 0..size {
        let li = l[i];
        for j in (i + 1)..size {
            if i == j {
                continue;
            }
            let lj = l[j];

            for k in (j + 1)..size {
                if k == j || k == i {
                    continue;
                }
                let lk = l[k];

                // eprintln!("sum: {}, {}, {}", li + lj, lj + lk, lk + li);
                if li + lj > lk && lj + lk > li && lk + li > lj {
                    if li != lj && lj != lk && lk != li {
                        // eprintln!("{},{},{}", li, lj, lk);
                        // eprintln!("{},{},{}", i + 1, j + 1, k + 1);
                        ans += 1;
                        // let mut sorted = [li, lj, lk];
                        // sorted.sort();
                        // set.insert((sorted[0], sorted[1], sorted[2]));
                    }
                }
            }
        }
    }

    // println!("{:?}", set);
    // println!("{}", set.len());
    println!("{}", ans);
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
