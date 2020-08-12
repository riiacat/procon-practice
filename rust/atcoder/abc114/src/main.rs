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

//abc152-D
#[fastout]
fn main() {
    input![n: u64];

    let mut comb = vec![0; 100];

    for i in 1..(n + 1) {
        if i % 10 == 0 {
            continue;
        }

        let d1 = i % 10;
        let mut d10 = i;
        while d10 > 9 {
            d10 /= 10;
        }

        let key = (d1 + d10 * 10) as usize;
        // eprintln!("i:{}, key; {}", i, key);
        comb[key] += 1;
    }

    let mut ans = 0;

    // let mut idx = 1;
    let mut is_visits = vec![false; 100];
    for (idx, c) in comb.iter().enumerate() {
        if is_visits[idx as usize] {
            continue;
        }

        if *c > 0 {
            // eprintln!(
            //     "idx + 1: {}, comb: {}, +{}",
            //     idx + 1,
            //     c,
            //     max(0, (c * (c - 1) / 2) + c as u64)
            // );

            let d1 = (idx) % 10;
            let mut d10 = idx;
            while d10 > 9 {
                d10 /= 10;
            }
            let inv_idx = (d1 * 10 + d10 * 1) as usize;

            ans += comb[inv_idx as usize] * c;
            // eprintln!(
            //     "idx: {}, inv_idx: {}, c: {}, c_inv: {}",
            //     idx, inv_idx, c, comb[inv_idx as usize]
            // );

            // is_visits[idx] = true;
            // is_visits[inv_idx] = true;
        }
    }

    println!("{}", ans);
}
