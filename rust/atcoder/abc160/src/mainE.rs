// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::BinaryHeap;
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

//abc160-E
#[fastout]
fn main() {
    input![
        X: usize,
        Y: usize,
        a: usize,
        b: usize,
        c: usize,
        p_input: [i64; a],
        q_input: [i64; b],
        r_input: [i64; c],
    ];

    let mut p = BinaryHeap::from(p_input);
    let mut q = BinaryHeap::from(q_input);
    let mut r = BinaryHeap::from(r_input);

    // let read_closure = |v: &mut BinaryHeap<i128>, count| {
    //     for _ in 0..count {
    //         v.push(read());
    //     }
    // };

    // read_closure(&mut p, a);
    // read_closure(&mut q, b);
    // read_closure(&mut r, c);

    let (mut x, mut y, mut z) = (0, 0, 0);

    let mut ans = 0;
    while x + y + z < X + Y {
        let a_max = if x == X { -1 } else { *p.peek().unwrap() };
        let b_max = if y == Y { -1 } else { *q.peek().unwrap() };
        let c_max = *r.peek().unwrap_or(&-1);

        if a_max >= b_max && a_max >= c_max {
            ans += a_max;
            x += 1;
            p.pop();
        } else if b_max >= c_max && b_max >= a_max {
            ans += b_max;
            y += 1;
            q.pop();
        } else if c_max >= a_max && c_max >= b_max {
            ans += c_max;
            z += 1;
            r.pop();
        }
    }

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
