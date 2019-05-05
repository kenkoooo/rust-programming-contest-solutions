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
use std::collections::VecDeque;

fn main() {
    input!(n: usize, k: usize, a: [usize1; n]);
    let mut ans = 0;
    let mut a = a;
    if a[0] != 0 {
        ans += 1;
        a[0] = 0;
    }
    let mut tree = vec![vec![]; n];
    for child in 0..n {
        let parent = a[child];
        tree[parent].push(child);
    }

    let mut dist_from_leaf = vec![0; n];
    let mut finished_count = vec![0; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if tree[i].is_empty() {
            q.push_back(i);
        }
    }

    while let Some(v) = q.pop_front() {
        let parent = a[v];
        finished_count[parent] += 1;
        if dist_from_leaf[v] == k - 1 {
            if parent != 0 {
                ans += 1;
            }
        } else {
            dist_from_leaf[parent] = cmp::max(dist_from_leaf[parent], dist_from_leaf[v] + 1);
        };

        if finished_count[parent] == tree[parent].len() {
            q.push_back(parent);
        }
    }

    println!("{}", ans);
}
