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

//abc146-C
#[fastout]
fn main() {
    input![a: u64, b: u64, x: u64];

    let price = |n| a * n + b * (log10(n as f64) as u64 + 1);

    //lower_bount - 1が正解
    let mut left_res = 0;
    let mut right_res = 10e8 as u64;
    let mut res = right_res / 2;

    if price(right_res) <= x {
        println!("{}", right_res);
        return;
    }

    loop {
        // eprintln!(
        //     "[{},{},{}], {}, {}",
        //     left_res,
        //     res,
        //     right_res,
        //     price(res),
        //     x
        // );
        let price = price(res);

        if price == x {
            println!("{}", min(res, 10e8 as u64));
            break;
        }

        if price < x {
            if right_res - 1 == res {
                println!("{}", min(res, 10e8 as u64));
                break;
            } else {
                //まだあげられる
                left_res = res;
                res = left_res + (right_res - left_res) / 2;
            }
        } else {
            if left_res + 1 == res || left_res + 1 == right_res {
                println!("{}", min(left_res, 10e8 as u64));
                break;
            } else {
                //まだ高い
                right_res = res;
                res = left_res + (right_res - left_res) / 2;
            }
        }
    }
}
