// -*- coding:utf-8-unix -*-

extern crate lazy_static;
extern crate num_bigint;
// 0.2.2
extern crate num_traits;

// 0.2.8
use num_bigint::BigInt;
use num_traits::{one, zero, Num, NumAssignOps, NumOps, One, Pow, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use itertools::{assert_equal, concat, Itertools};
use lazy_static::lazy_static;
// use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::*;
use std::mem::swap;
use std::ops::{BitAnd, Range, ShrAssign};
use std::str::FromStr;
use std::sync::Mutex;
use superslice::*;
use std::fmt::Debug;

// ##########
// read
// ###########
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

// ##########
// chmin, chmax
// https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
// ###########

#[allow(unused_macros)]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}



// ##########
// modint
// https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
// ##########

#[allow(dead_code)]
fn modinv<T: Num + NumAssignOps + NumOps + Copy + PartialOrd>(a: T, m: T) -> T {
    let mut a = a;
    let mut b = m;
    let mut u: T = one();
    let mut v: T = zero();

    while b != zero() {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m;
    if u < zero() {
        u += m;
    }
    return u;
}

#[test]
fn modinv_test() {
    assert_eq!(1, modinv(1, 13));
    assert_eq!(2, modinv(7, 13));
    assert_eq!(3, modinv(9, 13));
    assert_eq!(4, modinv(10, 13));
    assert_eq!(5, modinv(8, 13));
}

// long long modpow(long long a, long long n, long long mod) {
// long long res = 1;
// while (n > 0) {
// if (n & 1) res = res * a % mod;
// a = a * a % mod;
// n >>= 1;
// }
// return res;
// }
#[allow(dead_code)]
fn modpow<T>(a: T, n: T, modulo: T) -> T
where
    T: Num + NumAssignOps + NumOps + Copy + PartialOrd + BitAnd + PartialEq + ShrAssign,
    <T as BitAnd>::Output: PartialEq + Num,
{
    let mut res = one();
    let mut a = a;
    let mut n = n;
    while n > zero() {
        if (n & one()) == one() {
            res = res * a % modulo;
        }
        a = a * a % modulo;
        n >>= one();
    }

    return res;
}

#[test]
fn modpow_test() {
    assert_eq!(4, modpow(2, 2, 13));
    assert_eq!(3, modpow(2, 4, 13));
}

// 前処理 com_init(): O(n)
// クエリ処理 COM(n, k): O(1)
// conv::com_init();
// conv::com(n,k);
mod comb {
    use super::*;
    lazy_static! {
        static ref FAC: Mutex<Vec<usize>> = Mutex::default();
        static ref FINV: Mutex<Vec<usize>> = Mutex::default();
        static ref INV: Mutex<Vec<usize>> = Mutex::default();
        static ref MODULO: Mutex<usize> = Mutex::default();
        // static ref MAXNCONV: Mutex<usize> = Mutex::default();
    }

    // // テーブルを作る前処理
    // void com_init() {
    // fac[0] = fac[1] = 1;
    // finv[0] = finv[1] = 1;
    // inv[1] = 1;
    // for (int i = 2; i < MAX; i++){
    // fac[i] = fac[i - 1] * i % MOD;
    // inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
    // finv[i] = finv[i - 1] * inv[i] % MOD;
    // }

    #[allow(dead_code)]
    fn com_init_with(modulo: usize, maxn_conv: usize) {
        let mut fac = FAC.lock().unwrap();
        let mut finv = FINV.lock().unwrap();
        let mut inv = INV.lock().unwrap();
        *fac = vec![0; maxn_conv];
        *finv = vec![0; maxn_conv];
        *inv = vec![0; maxn_conv];

        let mut g_modulo = MODULO.lock().unwrap();
        *g_modulo = modulo;

        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;

        for i in 2..maxn_conv {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
    }

    #[allow(dead_code)]
    pub fn com_init() {
        com_init_with(MOD, MAXN_CONV);
    }

    // // 二項係数計算
    // long long COM(int n, int k){
    // if (n < k) return 0;
    // if (n < 0 || k < 0) return 0;
    // return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
    // }
    #[allow(dead_code)]
    pub fn com(n: usize, k: usize) -> usize {
        let fac = FAC.lock().unwrap();
        let finv = FINV.lock().unwrap();
        // let mut inv = INV.lock().unwrap();
        let m = *MODULO.lock().unwrap();

        if n < k {
            return 0;
        }
        // if n < 0 || k < 0 {
        //     return 0;
        // }
        return fac[n] * (finv[k] * finv[n - k] % m) % m;
    }

    #[test]
    fn com_test() {
        com_init_with(13, 100);
        assert_eq!(12, com(12, 1));
        assert_eq!(66 % 13, com(12, 2));
        assert_eq!(220 % 13, com(12, 3));
        assert_eq!(495 % 13, com(12, 4));
        assert_eq!(792 % 13, com(12, 5));
        assert_eq!(924 % 13, com(12, 6));
        assert_eq!(com(12, 5), com(12, 7));
    }
}

// ##########
// union-find
// http://sntea.hatenablog.com/entry/2017/06/07/091246
// ##########

mod uf {
    // let mut uf = uf::UnionFind::new(10);
    // uf.unite; uf.same
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct UnionFind {
    par: Vec<i64>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        #[allow(dead_code)]
        pub fn new(n: usize) -> UnionFind {
            let mut vec = vec![0; n];
            for i in 0..n {
                vec[i] = -1;
            }
            UnionFind {
                par: vec,
                rank: vec![0; n],
            }
        }

        #[allow(dead_code)]
        fn find(&mut self, x: usize) -> usize {
            if self.par[x] < 0 {
                x
            } else {
                let par = self.par[x];
                let res = self.find(par as usize);
                self.par[x] = res as i64;
                res
            }
        }

        #[allow(dead_code)]
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }

        #[allow(dead_code)]
        pub fn unite(&mut self, a: usize, b: usize) {
            let apar = self.find(a);
            let bpar = self.find(b);
            if self.rank[apar] > self.rank[bpar] {
                self.par[apar] += self.par[bpar];
                self.par[bpar] = apar as i64;
            } else {
                self.par[bpar] += self.par[apar];
                self.par[apar] = bpar as i64;
                if self.rank[apar] == self.rank[bpar] {
                    self.rank[bpar] += 1;
                }
            }
        }

        #[allow(dead_code)]
        pub fn size(&mut self, x: usize) -> usize {
            let parent = self.find(x);
            //parentのparにサイズが負の状態で入る
            return (-self.par[parent]) as usize;
        }
    }

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(10);

        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(i == j, uf.same(i, j));
            }
        }

        uf.unite(0, 1);
        assert_eq!(true, uf.same(0, 1));
        //false
        assert_eq!(false, uf.same(0, 9));
        assert_eq!(false, uf.same(1, 9));
        assert_eq!(false, uf.same(2, 9));

        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        //1
        assert_eq!(1, uf.size(2));
        assert_eq!(1, uf.size(8));
        assert_eq!(1, uf.size(9));

        uf.unite(8, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        //false
        assert_eq!(false, uf.same(0, 9));
        assert_eq!(false, uf.same(1, 9));
        assert_eq!(false, uf.same(2, 9));

        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        assert_eq!(2, uf.size(8));
        assert_eq!(2, uf.size(9));
        //1
        assert_eq!(1, uf.size(2));

        uf.unite(1, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        assert_eq!(true, uf.same(0, 8));
        assert_eq!(true, uf.same(0, 9));
        assert_eq!(true, uf.same(1, 8));
        assert_eq!(true, uf.same(1, 9));
        //false
        assert_eq!(false, uf.same(2, 9));

        assert_eq!(4, uf.size(0));
        assert_eq!(4, uf.size(1));
        assert_eq!(4, uf.size(8));
        assert_eq!(4, uf.size(9));
        //1
        assert_eq!(1, uf.size(2));
    }
}

// ###########
// seg_tree
// ant_book
// ###########
mod seg_tree {
    #[derive(Debug)]
    struct SegTree<T: Clone> {
        n: usize,
        dat: Vec<Option<T>>,
    }

    impl<T: Clone + std::fmt::Debug> SegTree<T> {
        #[allow(dead_code)]
        pub fn new(size: usize) -> SegTree<T> {
            let mut size_pow2 = 1;
            while size_pow2 < size {
                size_pow2 *= 2;
            }

            let dat: Vec<Option<T>> = vec![None; 2 * size_pow2 - 1];
            SegTree { n: size_pow2, dat }
        }

        #[allow(dead_code)]
        pub fn update<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &mut self,
            k: usize,
            a: T,
            update: F,
        ) {
            let mut k = k;
            k += self.n - 1;
            self.dat[k] = Some(a);
            while k > 0 {
                k = (k - 1) / 2;
                self.dat[k] = update(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
            }
        }

        #[allow(dead_code)]
        fn query_inner<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &self,
            selection_query: &F,
            a: usize,
            b: usize,
            k: usize,
            l: usize,
            r: usize,
        ) -> Option<T> {
            if r <= a || b <= l {
                // eprintln!("{}, {}, {}, {}, {:?}", a, b, l, r, "none");
                return None;
            }

            return if a <= l && r <= b {
                // eprintln!("{}, {}, {}, {}, {:?}", a, b, r, l, self.dat[k]);
                self.dat[k].clone()
            } else {
                let vl = self.query_inner(selection_query, a, b, k * 2 + 1, l, (l + r) / 2);
                let vr = self.query_inner(selection_query, a, b, k * 2 + 2, (l + r) / 2, r);

                selection_query(&vl, &vr)
            };
        }

        #[allow(dead_code)]
        pub fn query<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &self,
            selection_query: &F,
            a: usize,
            b: usize,
        ) -> Option<T> {
            return self.query_inner(selection_query, a, b, 0, 0, self.n);
        }
    }

    #[test]
    fn test_segtree_rmq() {
        let mut t: SegTree<usize> = SegTree::new(5);

        let cmp_f = |lhs: &Option<usize>, rhs: &Option<usize>| {
            if lhs.is_none() {
                return rhs.clone();
            }

            if rhs.is_none() {
                return lhs.clone();
            }

            return if lhs.unwrap() <= rhs.unwrap() {
                lhs.clone()
            } else {
                rhs.clone()
            };
        };
        // 1, 3, 2, 5, 1
        t.update(0, 1, cmp_f);
        t.update(1, 3, cmp_f);
        t.update(2, 2, cmp_f);
        t.update(3, 5, cmp_f);
        t.update(4, 1, cmp_f);
        // println!("{:?}", t);

        assert_eq!(1, t.query(&cmp_f, 0, 1).unwrap());
        assert_eq!(3, t.query(&cmp_f, 1, 2).unwrap());
        assert_eq!(2, t.query(&cmp_f, 2, 3).unwrap());
        assert_eq!(5, t.query(&cmp_f, 3, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 4, 5).unwrap());

        // len2
        assert_eq!(1, t.query(&cmp_f, 0, 2).unwrap());
        assert_eq!(2, t.query(&cmp_f, 1, 3).unwrap());
        assert_eq!(2, t.query(&cmp_f, 2, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 3, 5).unwrap());

        // len3
        assert_eq!(1, t.query(&cmp_f, 0, 3).unwrap());
        assert_eq!(2, t.query(&cmp_f, 1, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 2, 5).unwrap());

        // len4
        assert_eq!(1, t.query(&cmp_f, 0, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 1, 5).unwrap());

        // len5
        assert_eq!(1, t.query(&cmp_f, 0, 6).unwrap());
    }
}

// ##############
// rolling hash
// ###############
mod rolling_hash {
    use super::*;
    use ascii::{AsciiStr, AsciiString};
    use num_traits::AsPrimitive;

    fn contains_with(base: u64, a: &AsciiStr, b: &AsciiStr) -> bool {
        let (al, bl) = (a.len(), b.len());
        if al > bl {
            return false;
        }

        let mut t: u64 = 1;
        for _ in 0..al {
            t = t.wrapping_mul(base);
        }

        let (mut ah, mut bh): (u64, u64) = (0, 0);

        for i in 0..al {
            ah = ah.wrapping_mul(base) + a[i].as_byte() as u64;
        }
        for i in 0..al {
            bh = bh.wrapping_mul(base) + b[i].as_byte() as u64;
        }

        // eprintln!("{}, {}", ah, bh);

        for i in 0..=bl - al {
            if ah == bh {
                return true;
            }
            if i + al < bl {
                let mut add: i64 = b[i + al].as_byte().as_();
                add -= ((b[i].as_byte() as u64).wrapping_mul(t)) as i64;
                bh = (bh.wrapping_mul(base) as i64).wrapping_add(add) as u64;
            }
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn contains(a: &AsciiStr, b: &AsciiStr) -> bool {
        return contains_with(BASE_ROLLING_HASH, a, b);
    }

    #[test]
    fn contains_test() {
        const base: u64 = 1000_000_007;
        assert_eq!(
            false,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("a").unwrap()
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aaabca").unwrap()
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("aaaaaa").unwrap(),
                &AsciiString::from_str("aaaaaa").unwrap()
            )
        );

        assert_eq!(
            false,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aacbaa").unwrap()
            )
        )
    }

    fn overlap_last_and_head_with(base: u64, a: &AsciiStr, b: &AsciiStr) -> usize {
        let (al, bl) = (a.len(), b.len());

        let mut ans = 0;
        let (mut ah, mut bh, mut t): (u64, u64, u64) = (0, 0, 1);
        for i in 1..=min(al, bl) {
            ah = ah.wrapping_add((a[al - i].as_byte() as u64).wrapping_mul(t));
            bh = bh
                .wrapping_mul(base)
                .wrapping_add(b[i - 1].as_byte() as u64);

            if ah == bh {
                ans = i;
            }
            t = t.wrapping_mul(base);
        }

        return ans;
    }

    fn overlap_last_and_head(a: &AsciiStr, b: &AsciiStr) -> usize {
        return overlap_last_and_head_with(BASE_ROLLING_HASH, a, b);
    }

    #[test]
    fn overlap_test() {
        const base: u64 = 1000_000_007;
        assert_eq!(
            0,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("a").unwrap()
            )
        );

        assert_eq!(
            2,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("bca").unwrap()
            )
        );

        assert_eq!(
            5,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("hogefoobar").unwrap(),
                &AsciiString::from_str("oobarhoge").unwrap()
            )
        );
    }
}

// ##########
// lazy_static!
// ##########
// lazy_static! {
//     static ref H: Mutex<Vec<i32>> = Mutex::default();
//     static ref W: Mutex<Vec<i32>> = Mutex::default();
// }
// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);

// MOD, Combination関連に使う定数
#[allow(dead_code)]
const BASE_ROLLING_HASH: u64 = 1158187049;
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
const MAXN_CONV: usize = 510000;
#[allow(dead_code)]
const INF: usize = 1_234_789_567;



// abc142-E
// #[fastout]
fn main() {
    input![n: usize, m:usize];

    let mut aa = Vec::new();
    let mut bb = Vec::new();
    let mut cc= Vec::new();

    for _ in 0..m{
        input![a: usize, b:usize, c:[usize; b]];
        aa.push(a); bb.push(b); cc.push(c);
    }

    let cc_flag = cc.iter().map(
        |v| v.iter().fold(0, |c, f| c | (1 << (f-1) as usize))).collect_vec();

    // eprintln!("{:?}", cc_flag.iter().map(|b| format!("{:012b}", b)).collect_vec());

    let mut dp = vec![vec![INF; 2usize.pow(n as u32)]; m];

    for j in 0..2usize.pow(n as u32){
        dp[0][j] = if j & cc_flag[0] == j {aa[0]}else{INF};
    }
    dp[0][0] = 0;

    // eprintln!("{:?}", dp[0]);

    // for i in 1..m{
    //     for j in 0..2usize.pow(n as u32){
    //         chmin!(dp[i][j], dp[i-1][j]);
    //         let f = cc_flag[i];
    //         // eprintln!("{:012b}, {:012b}, {:012b}", j, f, (j ^ f) & (!f));
    //         chmin!(dp[i][j], dp[i-1][(j ^ f) & (!f)] + aa[i]);
    //     }
    // }
    //
    for i in 1..m{
        for j in 0..2usize.pow(n as u32){
            let f = cc_flag[i];
            chmin!(dp[i][j], dp[i-1][j]);
            chmin!(dp[i][j | f], dp[i-1][j | f]);
            // eprintln!("{:012b}, {:012b}, {:012b}", j, f, (j ^ f) & (!f));
            chmin!(dp[i][j | f], dp[i-1][j] + aa[i]);
        }
    }



    // eprintln!("{:?}", dp);

    //new type
    if dp[m-1][2usize.pow(n as u32)-1] >= INF {
        println!("-1");
    }else {
        println!("{}", dp[m-1][2usize.pow(n as u32)-1]);
    }
}
