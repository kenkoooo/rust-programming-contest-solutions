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

const MOD: usize = 1e9 as usize + 7;

fn solve(n: usize, k: usize, b: &Vec<usize>) -> usize {
    let mut dp = vec![vec![vec![0; 2]; k + 1]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in (0..(k + 1)).rev() {
            for t in 0..(b[i] + 1) {
                if j + t > k {
                    break;
                }
                dp[i + 1][j + t][1] += dp[i][j][1];
                dp[i + 1][j + t][1] %= MOD;
                if t == b[i] {
                    dp[i + 1][j + t][1] += dp[i][j][0];
                    dp[i + 1][j + t][1] %= MOD;
                } else {
                    dp[i + 1][j + t][0] += dp[i][j][0];
                    dp[i + 1][j + t][0] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..(k + 1) {
        ans += dp[n][i][1];
    }
    (ans + dp[n][k][0]) % MOD
}
fn main() {
    input!(n: usize, k: usize, a: [usize; n]);
    let mut b = vec![0; n];
    for i in 0..n {
        let mut a = a[i];
        let mut count = 0;
        while a > 0 {
            count += 1;
            a /= 2;
        }
        b[i] = count;
    }

    let k = cmp::min(k, b.iter().sum());
    let ans = solve(n, k, &b);
    println!("{}", ans);
}
