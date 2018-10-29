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
use std::cmp::Ordering;

fn main() {
    input!(r: f64, n: usize, m: usize);
    let mut d = vec![0.0; n + 1];
    for i in 0..n {
        let w = r * 2.0 / (n as f64);
        let dist = (r - w * (i as f64)).abs();
        d[i] = (r.powi(2) - dist.powi(2)).sqrt() * 2.0;
    }
    let mut used = vec![false; n + 1];
    let mut e = d
        .iter()
        .enumerate()
        .map(|(i, &d)| (d, i))
        .collect::<Vec<_>>();
    e.sort_by(|&(d1, _), &(d2, _)| {
        if d1 < d2 {
            Ordering::Greater
        } else if d1 > d2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let mut ans = 0.0;
    for &(_, i) in e.iter() {
        if used[i] {
            continue;
        }
        let mut p = n;
        for j in 0..n {
            if i == j {
                continue;
            }
            if used[i] {
                continue;
            }
            if cmp::min(i, j) + m > cmp::max(i, j) {
                continue;
            }
            if (d[i] - d[j]).abs() < (d[i] - d[p]).abs() {
                p = j;
            }
        }
        used[p] = true;
        used[i] = true;
        ans += if d[i] > d[p] { d[i] } else { d[p] };
    }

    println!("{}", ans);
}
