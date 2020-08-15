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

//abc042-C
#[fastout]
fn main() {
    let mut n: u64 = read();
    let k = read();

    let mut d = Vec::new();
    for _ in 0..k {
        d.push(read());
    }

    loop {
        let mut mid = n;
        let mut is_ok = true;
        while mid > 0 {
            let remaind = mid % 10;
            if d.contains(&remaind) {
                is_ok = false;
                break;
            }

            mid /= 10;
        }

        if is_ok {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}
