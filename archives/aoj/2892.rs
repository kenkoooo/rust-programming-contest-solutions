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
    input!(n: usize, words: [chars; n]);
    let mut jump = vec![vec![]; 26];
    let mut heads = vec![0; n];
    for i in 0..n {
        let c = words[i][0] as usize - 'a' as usize;
        jump[c].push(i);
        heads[i] = c;
    }

    let mut next = vec![1; 26];
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let c = heads[i];
        dp[i + 1] = cmp::min(dp[i + 1], dp[i] + 1);
        if next[c] < jump[c].len() {
            let jump = jump[c][next[c]];
            dp[jump] = cmp::min(dp[jump], dp[i]);
            next[c] += 1;
        }
    }
    println!("{}", dp[n]);
}
