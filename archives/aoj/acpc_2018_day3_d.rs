/// Thank you tanakh!!!
///  https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

const INF: usize = 1 << 30;

fn main() {
    input!(n: usize, s: [chars; n]);
    let mut dictionary = vec![vec![]; 26];
    for i in 0..n {
        let c = s[i][0] as usize - 'a' as usize;
        dictionary[c].push(i);
    }

    let mut jump = vec![INF; n];
    for i in 0..26 {
        for j in 1..dictionary[i].len() {
            jump[dictionary[i][j - 1]] = dictionary[i][j];
        }
    }

    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        dp[i + 1] = cmp::min(dp[i + 1], dp[i] + 1);
        if jump[i] != INF {
            dp[jump[i]] = cmp::min(dp[jump[i]], dp[i]);
        }
    }

    println!("{}", dp[n]);
}
