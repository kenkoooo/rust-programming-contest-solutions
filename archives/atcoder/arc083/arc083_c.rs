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

const INF: usize = 1e15 as usize;

fn main() {
    input!(n: usize, p: [usize1; n - 1], x: [usize; n]);
    let mut tree = vec![vec![]; n];
    for child in 1..n {
        let parent = p[child - 1];
        tree[parent].push(child);
    }
    if dfs(0, &tree, &x) != INF {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn dfs(v: usize, tree: &Vec<Vec<usize>>, x: &Vec<usize>) -> usize {
    let mut dp = vec![INF; x[v] + 1];
    dp[0] = 0;
    for &child in tree[v].iter() {
        let mut next = vec![INF; x[v] + 1];
        let white = x[child];
        let black = dfs(child, tree, x);
        for i in 0..(x[v] + 1) {
            if i + white <= x[v] {
                next[i + white] = cmp::min(next[i + white], dp[i] + black);
            }
            if i + black <= x[v] {
                next[i + black] = cmp::min(next[i + black], dp[i] + white);
            }
        }
        dp = next;
    }
    *dp.iter().min().unwrap()
}
