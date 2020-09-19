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

//abc099-C
#[fastout]
fn main() {
    let n: i64 = read();

    let mut ansv = vec![10000000; (n + 1) as usize];

    {
        let mut num = 1;
        while num as i64 <= n {
            // eprintln!("{} = 1", num);
            ansv[num] = 1;
            num *= 6;
        }
    }

    {
        let mut num = 1;
        while num as i64 <= n {
            // eprintln!("{} = 1", num);
            ansv[num] = 1;
            num *= 9;
        }
    }

    for i in 2..(n + 1) {
        if ansv[i as usize] == 1 {
            // eprintln!("ansv[{}] = {}", i, 1);
            continue;
        }

        let a1 = ansv[(i - 1) as usize];
        let mut a6 = 1000000;
        {
            let mut sub = 6;
            while i - sub >= 1 {
                let mid = ansv[(i - sub) as usize];
                a6 = min(mid, a6);
                sub *= 6;
            }
        }

        let mut a9 = 1000000;
        {
            let mut sub = 9;
            while (i - sub >= 1) {
                let mid = ansv[(i - sub) as usize];
                a9 = min(a9, mid);
                sub *= 9;
            }
        }

        let new_min = min(min(a1, a6), a9) + 1;
        // eprintln!("ansv[{}] = {}", i, new_min);
        ansv[i as usize] = new_min;
    }

    println!("{}", ansv[n as usize]);
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}
