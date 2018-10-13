/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::cmp;
use std::collections::BinaryHeap;

fn solve(ab: &Vec<(usize, usize)>) -> usize {
    let n = ab.len();
    let mut v = vec![];
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((a, i));
        v.push((b, i));
    }
    v.sort();

    let mut used_count = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        let (x, i) = v[i];
        used_count[i] += 1;
        ans += x;
    }

    for i in 0..n {
        if used_count[i] == 2 {
            return ans;
        }
    }

    let mut min = 1e15 as usize;

    for i in 0..n {
        assert_eq!(used_count[i], 1);
        let (a, b) = ab[i];
        let ans = ans - cmp::min(a, b);
        let (w, j) = v[n];
        if j == i {
            let (w, _) = v[n + 1];
            min = cmp::min(min, ans + w);
        } else {
            min = cmp::min(min, ans + w);
        }
    }
    min
}

fn main() {
    input!(n: usize, ab: [(usize, usize); n]);

    let ans0: usize = ab.iter().map(|&(a, _)| a).sum();
    let ans1: usize = ab.iter().map(|&(_, b)| b).sum();
    let ans2 = solve(&ab);
    println!("{}", cmp::min(cmp::min(ans0, ans1), ans2));
}
