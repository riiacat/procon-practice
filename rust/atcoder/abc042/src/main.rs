// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;

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

//abc114-C
// #[fastout]
fn main() {
    input![n_input: usize];
    n_input;

    let mut num753_cand: Vec<usize> = Vec::new();

    go(0, 0, 3, &mut num753_cand, n_input);
    go(0, 0, 5, &mut num753_cand, n_input);
    go(0, 0, 7, &mut num753_cand, n_input);

    let mut ans = 0;
    for i in num753_cand.iter() {
        let i_str = i.to_string();

        let mut is_753 = true;
        let mut counts = vec![0; 3];
        for c in i_str.chars() {
            if c != '7' || c != '5' || c != '3' {}

            match c {
                '7' => counts[0] += 1,
                '5' => counts[1] += 1,
                '3' => counts[2] += 1,
                _ => {
                    is_753 = false;
                    break;
                }
            }
        }

        if !is_753 {
            continue;
        }

        if counts.iter().all(|c| *c > 0) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {
    if mid > n {
        return;
    }

    let mut fact = 1;
    for _ in 0..d {
        fact *= 10;
    }

    let plus = fact * use_num;
    let new_mid = mid + plus;

    if new_mid > n {
        return;
    }

    num753_cand.push(new_mid);

    go(new_mid, d + 1, 3, num753_cand, n);
    go(new_mid, d + 1, 5, num753_cand, n);
    go(new_mid, d + 1, 7, num753_cand, n);
}
