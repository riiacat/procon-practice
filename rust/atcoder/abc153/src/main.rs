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
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::*;
use std::ops::Range;
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

//abc175-D
// #[fastout]
fn main() {
    input![n: usize, k: usize, p: [usize; n], c: [i128; n]];
    let p: Vec<_> = p.iter().map(|a| a - 1).collect();

    let mut cycles = Vec::new();
    let mut is_visited = vec![false; n];

    while !is_visited.iter().all(|&a| a) {
        // eprintln!("new cycle before {:?}", cycles);
        let new_cycle = Vec::new();
        cycles.push(new_cycle);

        let (head_idx, _) = is_visited
            .iter()
            .enumerate()
            .find(|(_idx, is_visit)| !*is_visit)
            .unwrap();
        is_visited[head_idx] = true;
        let new_cycle = cycles.last_mut().unwrap();
        new_cycle.push(head_idx);

        let mut current = head_idx;
        loop {
            let next = p[current];
            if next == head_idx {
                break;
            }
            new_cycle.push(next);
            is_visited[next] = true;
            current = next;
        }
    }

    //eprintln!("cycles: {:?}", cycles);

    let mut maxes = Vec::new();
    for cycle in cycles.iter() {
        let can_cycle_num = (k / cycle.len()) as i128;
        let mut remaind = k % cycle.len();
        let cycle_sum: i128 = cycle.iter().map(|x| c[*x]).sum();
        // eprintln!(
        //     "cycle_sum: {:?}, cycle: {:?}, cs: {:?}",
        //     cycle_sum,
        //     cycle,
        //     cycle.iter().map(|x| c[*x]).collect::<Vec<_>>()
        // );

        let mut max_incycle: i128 = 0;
        if cycle_sum > 0 {
            max_incycle += can_cycle_num * cycle_sum;
        }

        let mut max_of_range = None;
        if can_cycle_num > 0 && remaind == 0 {
            remaind = cycle.len();
        }

        for s in 0..cycle.len() {
            let mut sum_ranges = vec![0; remaind];
            // let mut sum_range = 0;
            for i in s..(s + remaind) {
                if i - s == 0 {
                    sum_ranges[i - s] = c[cycle[i % cycle.len()]];
                } else {
                    // eprintln!("{},{}", i - s, i % cycle.len());
                    sum_ranges[i - s] = sum_ranges[i - s - 1] + c[cycle[i % cycle.len()]];
                }
            }
            //eprintln!("{}, {}, sum_range: {}", l, s, sum_range);
            max_of_range = if let Some(max_of_range) = max_of_range {
                Some(max(*sum_ranges.iter().max().unwrap(), max_of_range))
            } else {
                Some(*sum_ranges.iter().max().unwrap())
            };
        }

        max_incycle += max_of_range.unwrap();
        maxes.push(max_incycle);
    }

    //eprintln!("maxes: {:?}", maxes);
    println!("{}", maxes.iter().max().unwrap());
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

// -100000000
//  1000000000
