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

const MOD: usize = 1e9 as usize + 7;

fn main() {
    input!(n: usize, k: usize, a: [usize; n]);
    let mut count = vec![0; n];
    for i in 0..n {
        let mut a = a[i];
        while a > 0 {
            a /= 2;
            count[i] += 1;
        }
    }
    let sum = count.iter().sum::<usize>();
    let mut dp = vec![vec![vec![0; 2]; sum + 1]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..dp[i].len() {
            for c in 0..(count[i] + 1) {
                for mark in 0..2 {
                    if j + c > sum {
                        break;
                    }
                    let next_mark = if mark == 1 || c == count[i] { 1 } else { 0 };
                    dp[i + 1][j + c][next_mark] += dp[i][j][mark];
                    dp[i + 1][j + c][next_mark] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..dp[n].len() {
        if i > k {
            break;
        }
        for m in 0..2 {
            if i < k && m != 1 {
                continue;
            }
            ans += dp[n][i][m];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
