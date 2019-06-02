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
use std::collections::BTreeSet;

fn main() {
    input!(s: chars);
    let n = s.len();
    let s = s.iter().map(|&c| c == '1').collect::<Vec<_>>();
    if !s[0] || s[n - 1] {
        println!("-1");
        return;
    }

    let mut tree = vec![vec![]; n];
    let mut cur = 0;
    let mut cur_size = 1;

    for i in 0..(n - 1) {
        let one = i + 1;
        let other = n - one;
        if s[one - 1] != s[other - 1] {
            println!("-1");
            return;
        }
        if !s[i] {
            continue;
        }

        let have_to_add = one - cur_size + 1;
        for _ in 0..have_to_add {
            if cur_size == n {
                println!("-1");
                return;
            }
            let v = cur_size;
            tree[cur].push(v);
            tree[v].push(cur);
            cur_size += 1;
        }
        cur = cur_size - 1;
    }

    let mut set = BTreeSet::new();
    for from in 0..n {
        for &to in tree[from].iter() {
            let (a, b) = (cmp::min(from, to), cmp::max(from, to));
            set.insert((a, b));
        }
    }

    assert_eq!(set.len(), n - 1);
    for &(a, b) in set.iter() {
        println!("{} {}", a + 1, b + 1);
    }
}
