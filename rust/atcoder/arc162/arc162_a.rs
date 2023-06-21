// -*- coding:utf-8-unix -*-

extern crate lazy_static;

use ascii::AsciiString;
use fixedbitset::Union;
use itertools::Permutations;
use num::Integer;
use num::abs;
use itertools::CombinationsWithReplacement;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
// 0.2.8
use num_bigint::BigInt;
use num_integer::ExtendedGcd;
use num_integer::Roots;
use num_integer::div_mod_floor;
use num_integer::gcd;
use num_integer::gcd_lcm;
use num_traits::{one, zero, Num, NumAssignOps, NumOps, One, Pow, ToPrimitive, Zero};

use petgraph::algo::dijkstra;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::UnGraph;
// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
// use std::convert::TryInto;
use itertools::{assert_equal, concat};
use lazy_static::lazy_static;
// use libm::*;
use ascii::AsciiChar;
use proconio::marker::Usize1;
use proconio::source::line::LineSource;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::*;
use std::collections::BTreeSet;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::default;
use std::f32::EPSILON;
use std::f64::consts::PI;
use std::fmt::Binary;
use std::io;
use std::io::*;
use std::iter::FromIterator;
use std::mem::swap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::{BitAnd, Neg, Range, ShrAssign};
use std::str::FromStr;
use std::sync::Mutex;
use std::vec;
use superslice::*;

use crate::comb::com;
use crate::comb::com_init;
use crate::prioirty_sum_structure::PSum;
use crate::seg_tree::SegTree;
use crate::uf::UnionFind;

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
fn modinv<T: Num + NumAssignOps + NumOps + Copy + PartialOrd + Neg>(a: T, m: T) -> T {
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
        pub fn find(&mut self, x: usize) -> usize {
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
            if self.same(a, b){return ;}

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
    pub struct SegTree<T: Clone> {
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
                &AsciiString::from_str("a").unwrap(),
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aaabca").unwrap(),
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("aaaaaa").unwrap(),
                &AsciiString::from_str("aaaaaa").unwrap(),
            )
        );

        assert_eq!(
            false,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aacbaa").unwrap(),
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
                &AsciiString::from_str("a").unwrap(),
            )
        );

        assert_eq!(
            2,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("bca").unwrap(),
            )
        );

        assert_eq!(
            5,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("hogefoobar").unwrap(),
                &AsciiString::from_str("oobarhoge").unwrap(),
            )
        );
    }
}

mod nxt_perm{
    //! 順列。

    /// 辞書順で次の順列の生成。
    ///
    /// # Idea
    /// `todo!()`
    ///
    /// # References
    /// - <https://stackoverflow.com/questions/11483060>
    ///
    /// # Examples
    /// ```
    /// use nekolib::algo::next_permutation;
    ///
    /// let mut a = vec![1, 3, 2];
    /// assert!(next_permutation(&mut a));
    /// assert_eq!(a, [2, 1, 3]);
    ///
    /// // last one
    /// let mut a = vec![3, 2];
    /// assert!(!next_permutation(&mut a));
    /// assert_eq!(a, [2, 3]);
    ///
    /// // empty one
    /// let mut a = Vec::<()>::new();
    /// assert!(!next_permutation(&mut a));
    ///
    /// // duplicated one
    /// let mut a = vec![1, 3, 2, 2, 3];
    /// next_permutation(&mut a);
    /// assert_eq!(a, [1, 3, 2, 3, 2]);
    pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
        let n = a.len();
        if n <= 1 {
            return false;
        }

        for i in (0..n - 1).rev() {
            if a[i] < a[i + 1] {
                let j = (0..n).rev().find(|&j| a[i] < a[j]).unwrap();
                a.swap(i, j);
                a[i + 1..].reverse();
                return true;
            }
        }

        a.reverse();
        false
    }

    pub fn prev_permutation<T: Ord>(a: &mut [T]) -> bool {
        let n = a.len();
        if n <= 1 {
            return false;
        }

        for i in (0..n - 1).rev() {
            if a[i] > a[i + 1] {
                let j = (0..n).rev().find(|&j| a[i] > a[j]).unwrap();
                a.swap(i, j);
                a[i + 1..].reverse();
                return true;
            }
        }

        a.reverse();
        false
    }

    fn next_permutation_with_count<T: Ord>(a: &mut [T], k: usize) -> bool {
        // precondition: k <= a.len(), and a[k..] is sorted
        // postcondition: a[k..] is sorted

        let n = a.len();
        for i in (0..k).rev() {
            let j = if k < n && a[i] < a[n - 1] {
                (k..).find(|&j| a[i] < a[j]).unwrap()
            } else if i + 1 < n && a[i] < a[i + 1] {
                (i..k).rev().find(|&j| a[i] < a[j]).unwrap()
            } else {
                continue;
            };
            a.swap(i, j);
            a[i + 1..k].reverse();
            a[i + 1..].rotate_right(n - k);
            return true;
        }
        false
    }

    fn prev_permutation_with_count<T: Ord>(a: &mut [T], k: usize) -> bool {
        // precondition: k <= a.len(), and a[k..] is reversely sorted
        // postcondition: a[k..] is reversely sorted

        let n = a.len();
        for i in (0..k).rev() {
            let j = if k < n && a[i] > a[n - 1] {
                (k..).find(|&j| a[i] > a[j]).unwrap()
            } else if i + 1 < n && a[i] > a[i + 1] {
                (i..k).rev().find(|&j| a[i] > a[j]).unwrap()
            } else {
                continue;
            };
            a.swap(i, j);
            a[i + 1..k].reverse();
            a[i + 1..].rotate_right(n - k);
            return true;
        }
        false
    }

    pub struct Permutations<T>(Vec<T>);

    impl<T: Ord> From<Vec<T>> for Permutations<T> {
        fn from(buf: Vec<T>) -> Self { Self(buf) }
    }

    #[allow(dead_code)]
    impl<T: Ord> Permutations<T> {
        pub fn next(&mut self) -> bool { next_permutation(&mut self.0) }
        pub fn prev(&mut self) -> bool { prev_permutation(&mut self.0) }
        pub fn peek(&self) -> &[T] { &self.0 }
    }


    impl<T: Ord + Clone> Permutations<T> {
        #[allow(dead_code)]
        pub fn forward(self, count: usize) -> Forward<T> {
            Forward::new(self, count)
        }
        #[allow(dead_code)]
        pub fn backward(self, count: usize) -> Backward<T> {
            Backward::new(self, count)
        }
    }

    pub struct Forward<T> {
        buf: Vec<T>,
        count: usize,
        finish: bool,
    }

    #[allow(dead_code)]
    impl<T: Clone + Ord> Forward<T> {
        fn new(perm: Permutations<T>, count: usize) -> Self {
            let mut buf = perm.0;
            buf[count..].sort();
            Self { buf, count, finish: false }
        }
    }

    impl<T: Clone + Ord> Iterator for Forward<T> {
        type Item = Vec<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.finish {
                None
            } else {
                let tmp = self.buf[..self.count].to_vec();
                self.finish =
                    !next_permutation_with_count(&mut self.buf, self.count);
                Some(tmp)
            }
        }
    }

    pub struct Backward<T> {
        buf: Vec<T>,
        count: usize,
        finish: bool,
    }

    #[allow(dead_code)]
    impl<T: Clone + Ord> Backward<T> {
        fn new(perm: Permutations<T>, count: usize) -> Self {
            let mut buf = perm.0;
            buf[count..].sort_by(|x, y| y.cmp(x));
            Self { buf, count, finish: false }
        }
    }

    impl<T: Clone + Ord> Iterator for Backward<T> {
        type Item = Vec<T>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.finish {
                None
            } else {
                let tmp = self.buf[..self.count].to_vec();
                self.finish =
                    !prev_permutation_with_count(&mut self.buf, self.count);
                Some(tmp)
            }
        }
    }

    #[test]
    fn iter() {
        let expected = vec![
            vec![0, 1, 1, 2, 2],
            vec![0, 1, 2, 1, 2],
            vec![0, 1, 2, 2, 1],
            vec![0, 2, 1, 1, 2],
            vec![0, 2, 1, 2, 1],
            vec![0, 2, 2, 1, 1],
            vec![1, 0, 1, 2, 2],
            vec![1, 0, 2, 1, 2],
            vec![1, 0, 2, 2, 1],
            vec![1, 1, 0, 2, 2],
            vec![1, 1, 2, 0, 2],
            vec![1, 1, 2, 2, 0],
            vec![1, 2, 0, 1, 2],
            vec![1, 2, 0, 2, 1],
            vec![1, 2, 1, 0, 2],
            vec![1, 2, 1, 2, 0],
            vec![1, 2, 2, 0, 1],
            vec![1, 2, 2, 1, 0],
            vec![2, 0, 1, 1, 2],
            vec![2, 0, 1, 2, 1],
            vec![2, 0, 2, 1, 1],
            vec![2, 1, 0, 1, 2],
            vec![2, 1, 0, 2, 1],
            vec![2, 1, 1, 0, 2],
            vec![2, 1, 1, 2, 0],
            vec![2, 1, 2, 0, 1],
            vec![2, 1, 2, 1, 0],
            vec![2, 2, 0, 1, 1],
            vec![2, 2, 1, 0, 1],
            vec![2, 2, 1, 1, 0],
        ];

        let fwd = Permutations::from(expected[0].clone()).forward(5);
        assert!(fwd.eq(expected.iter().cloned()));

        let fwd_1 = Permutations::from(expected[1].clone()).forward(5);
        assert!(fwd_1.eq(expected[1..].iter().cloned()));

        let fwd_28 = Permutations::from(expected[28].clone()).forward(5);
        assert!(fwd_28.eq(expected[28..].iter().cloned()));

        let fwd_29 = Permutations::from(expected[29].clone()).forward(5);
        assert!(fwd_29.eq(expected[29..].iter().cloned()));

        let bwd = Permutations::from(expected[0].clone()).backward(5);
        assert!(bwd.eq(expected[..=0].iter().rev().cloned()));

        let bwd_1 = Permutations::from(expected[1].clone()).backward(5);
        assert!(bwd_1.eq(expected[..=1].iter().rev().cloned()));

        let bwd_28 = Permutations::from(expected[28].clone()).backward(5);
        assert!(bwd_28.eq(expected[..=28].iter().rev().cloned()));

        let bwd_29 = Permutations::from(expected[29].clone()).backward(5);
        assert!(bwd_29.eq(expected[..=29].iter().rev().cloned()));
    }

    #[test]
    fn empty() {
        let empty: Vec<()> = vec![];
        let fwd = Permutations::from(empty.clone()).forward(0);
        assert!(fwd.eq(Some(vec![])));
        let bwd = Permutations::from(empty.clone()).backward(0);
        assert!(bwd.eq(Some(vec![])));
    }

    #[test]
    fn single() {
        let empty = vec![()];
        let fwd = Permutations::from(empty.clone()).forward(1);
        assert!(fwd.eq(Some(vec![()])));
        let bwd = Permutations::from(empty.clone()).backward(1);
        assert!(bwd.eq(Some(vec![()])));
    }

    #[cfg(test)]
    fn is_sorted<T: Ord>(a: &[T]) -> bool { a.windows(2).all(|w| w[0] <= w[1]) }

    #[test]
    fn partial_fwd() {
        for n in 1..=8 {
            for k in 1..=n {
                let a: Vec<_> = (0..n).collect();
                let expected: Vec<_> = Permutations::from(a.clone())
                    .forward(n)
                    .filter(|p| is_sorted(&p[k..]))
                    .map(|p| p[..k].to_vec())
                    .collect();

                assert!(Permutations::from(a).forward(k).eq(expected));
            }
        }
    }

    #[test]
    fn partial_bwd() {
        for n in 1..=8 {
            for k in 1..=n {
                let a: Vec<_> = (0..n).collect();
                let expected: Vec<_> = Permutations::from(a.clone())
                    .forward(n)
                    .filter(|p| is_sorted(&p[k..]))
                    .map(|p| p[..k].to_vec())
                    .collect();
                let expected: Vec<_> = expected.into_iter().rev().collect();

                let a_rev: Vec<_> = a.into_iter().rev().collect();
                assert!(Permutations::from(a_rev).backward(k).eq(expected));
            }
        }
    }

    #[test]
    fn partial_dup() {
        let a = vec![0, 1, 1, 2, 2, 3, 3, 3];
        for n in 1..=8 {
            for k in 1..=n {
                let a: Vec<_> = a[..n].to_vec();
                let expected: Vec<_> = Permutations::from(a.clone())
                    .forward(n)
                    .filter(|p| is_sorted(&p[k..]))
                    .map(|p| p[..k].to_vec())
                    .collect();

                assert!(Permutations::from(a).forward(k).eq(expected));
            }
        }
    }
}

mod prioirty_sum_structure{
    use std::{collections::BinaryHeap, cmp::Reverse, ops::{AddAssign, SubAssign}};

    use num::{Zero, zero};

    pub struct PSum<T>{
        //Minimum sum
        iin: BinaryHeap<T>,
        d_in: BinaryHeap<T>,
        out: BinaryHeap<Reverse<T>>,
        d_out: BinaryHeap<Reverse<T>>,

        k: usize,
        sum: T,
    }

    impl<T> PSum<T> where
        T: Ord + Zero + AddAssign + Copy + SubAssign
    {
        #[allow(dead_code)]
        pub fn new(k: usize) -> Self{
            PSum{
                iin: BinaryHeap::new(),
                d_in: BinaryHeap::new(),
                out: BinaryHeap::new(),
                d_out: BinaryHeap::new(),

                k,
                sum: zero(),
            }
        }


        fn update(&mut self){
            while(self.iin.len() - self.d_in.len() < self.k && !self.out.is_empty()){
                let p = self.out.pop().unwrap().0;
                if !self.d_out.is_empty() && p == self.d_out.peek().unwrap().0 {
                    self.d_out.pop().unwrap();
                }else{
                    self.sum += p;
                    self.iin.push(p);
                }
            }

            while self.iin.len() - self.d_in.len() > self.k {
                let p = self.iin.pop().unwrap();
                if !self.d_in.is_empty() && p == *self.d_in.peek().unwrap(){
                    self.d_in.pop().unwrap();
                }else{
                    self.sum -= p;
                    self.out.push(Reverse(p));
                }
            }

            while !self.d_in.is_empty() && *self.iin.peek().unwrap() == *self.d_in.peek().unwrap() {
                self.iin.pop().unwrap();
                self.d_in.pop().unwrap();
            }
        }

        #[allow(dead_code)]
        pub fn get_sum(&self) -> T{
            return self.sum;
        }

        #[allow(dead_code)]
        pub fn insert(&mut self, x: T){
            self.iin.push(x);
            self.sum += x;
            self.update();
        }

        #[allow(dead_code)]
        pub fn erase(&mut self, x: T){
            assert!(self.size() > 0);
            if !self.iin.is_empty() && *self.iin.peek().unwrap() ==x {
                self.sum -= x;
                self.iin.pop().unwrap();
            }else if !self.iin.is_empty() && *self.iin.peek().unwrap() > x{
            // }else if !self.iin.is_empty() && *self.iin.peek().unwrap() < x{
                self.sum -= x;
                self.d_in.push(x);
            }else{
                self.d_out.push(Reverse(x));
            }
            self.update()
        }

        #[allow(dead_code)]
        pub fn set_k(&mut self, k: usize){
            self.k = k;
            self.update();
        }

        #[allow(dead_code)]
        pub fn get_k(&self)->usize{self.k}


        pub fn size(&self) -> usize{
            self.iin.len() + self.out.len() - self.d_in.len()-self.d_out.len()
        }
    }

}

#[allow(dead_code)]
fn to_alphabet_num(a: AsciiChar) -> usize {
    (a.as_byte() - AsciiChar::a.as_byte()) as usize
}

#[allow(dead_code)]
fn num_to_alphabet(a: usize) -> Option<AsciiChar> {
    let a = a
        .to_u8()
        .map(|a| AsciiChar::from_ascii(AsciiChar::a.as_byte() + a as u8).ok());
    return a.flatten();
}

#[allow(dead_code)]
fn matmul<T: Mul<Output=T>+Rem<Output=T>+Add+AddAssign+Default+Copy>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>, m: T) -> Vec<Vec<T>> {
    let n = a.len();
    let mut res:Vec<Vec<T>> = vec![vec![Default::default(); n]; n];

    for i in 0..n{
        for j in 0..n{
            for k in 0..n{
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] = res[i][j] % m;
            }
        }
    }

    res
}


#[allow(dead_code)]
fn matpow<T: Mul<Output=T>+Rem<Output=T>+Add+AddAssign+Default+Copy+One>(a: &Vec<Vec<T>>, b: usize, m: T) -> Vec<Vec<T>> {
    let n = a.len();
    let mut a = a.clone();
    let mut res:Vec<Vec<T>> = vec![vec![Default::default(); n]; n];
    let mut b =b;
    for i in 0..n{res[i][i]=one();}
    while b>0{
        if (b&1)!=0 {res=matmul(&res, &a, m)}
        a = matmul(&a, &a, m);
        b >>=1;
    }
    res
}



// ##########
// lazy_static!
// ##########
lazy_static! {
    // static ref H: Mutex<Vec<i32>> = Mutex::default();
    // static ref W: Mutex<Vec<i32>> = Mutex::default();
}
// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);

// MOD, Combination関連に使う定数
#[allow(dead_code)]
const BASE_ROLLING_HASH: u64 = 1158187049;
#[allow(dead_code)]
// const MOD: usize = 998244353;
const MOD: usize = 1_000_000_007;
#[allow(dead_code)]
const MAXN_CONV: usize = 510000;

// let mut test_vec = vec![1, 3, 5, 7];
// let mut test_set = HashSet::new();
// let mut test_dict = HashMap::new();

// test_dict.insert("me", 1);
// let a = test_dict["me"];
// test_set.insert("value");
// {
//     let mut a = N.lock().unwrap();
//     *a = 10;
// }
// println!("{:?}", N.lock().unwrap());

#[allow(dead_code)]
fn idx2alphabet(i: usize) -> AsciiChar{
    AsciiChar::from_ascii((i as u8)+97).unwrap()
}
#[allow(dead_code)]
fn alphabet2idx(a: AsciiChar) -> usize{
    (a.as_byte()-97) as usize
}

#[allow(dead_code)]
fn get_runlength_encoding<T: Eq+Clone>(raw: &Vec<T>) -> Vec<(T, usize)>{
    let mut v = vec![];
    let mut cnt = 0_usize;

    let mut old: Option<T> = None;
    for i in 0..raw.len(){
        if let Some(oldv) = old.as_ref(){
            if *oldv == raw[i]{
                cnt+=1;
            }else{
                v.push((oldv.clone(), cnt));
                old = Some(raw[i].clone());
                cnt=1;
            }
        }else{
            cnt+=1;
            old=Some(raw[i].clone());
        }
    }
    if old.is_some(){
        v.push((old.unwrap(), cnt));
    }

    v
}


//TODO
// https://rsk0315.github.io/library-rs/nekolib/algo/permutation/index.html

// #[fastout]
fn main(){
    /*
     */
    // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    // println!("Hello, world!");
    input![t: usize, ];

    for _ in 0..t{
        input![n: usize, p: [usize; n]];

        let mut res = HashSet::new();
        for i in 0..n{res.insert(i);}

        let mut poss = vec![0; n];
        for i in 0..n{
            poss[i] = p[i];
        }

        // eprintln!("{:?}", poss);


        for i in 0..n{
            // if poss[i] > i {res.remove(&i);}

            // if poss[i] != 0{continue;}
            // {eprintln!("calc for {:?}", i);}
            // let mut ng = false;
            // let mut u = vec![];
            // for j in i+1..n{
            //     if poss[j] < poss[i]{
            //         // #[cfg(debug_assertions)]{eprintln!("{:?}", (i+1, j+1, poss[j]+1));}
            //         // {eprintln!("{:?}", (i+1, j+1, poss[j]+1));}
            //         // res.remove(&i);
            //         // ng = true;
            //         u.push(j);
            //     }
            // }

            let mut ng2 = false;
            // let mut v = vec![];
            for j in i+1..n{
                if poss[j] < poss[i]{
                    // #[cfg(debug_assertions)]{eprintln!("{:?}", (i+1, j+1, poss[j]+1));}
                    // {eprintln!("{:?}", (i+1, j+1, poss[j]+1));}
                    // res.remove(&i);
                    ng2 = true;
                }
            }

            // if ng&&ng2{
            if ng2{
                res.remove(&i);
                res.remove(&i);
            }
            // if poss[i] > i{ng=true}
        }

        // {
        //     let mut v = res.iter().map(|a| *a+1).collect_vec();
        //     v.sort();
        //     eprintln!("res: {:?}", v);
        // }


        println!("{}", res.len());
    }


    // println!("{}", res);
    // println!("{} {}", amax, res);
}

// #[cfg(debug_assertions)]{eprintln!("{:?}", 111.extended_gcd(&30));}

// fn tree_dfs(cur: usize, par: usize, adjs: &Vec<Vec<usize>>, ) {
// }
