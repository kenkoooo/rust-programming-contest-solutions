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

fn main() {
    input!(n: usize, s: chars);
    let mut ok = vec![false; n];
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for i in 1..(n - 1) {
        if s[i - 1] == '1' && s[i] == '0' && s[i + 1] == '1' {
            ok[i] = true;

            let mut cur = i - 1;
            while cur > 0 && s[cur - 1] == '1' {
                left[i] += 1;
                cur -= 1;
            }

            let mut cur = i + 1;
            while cur + 1 < n && s[cur + 1] == '1' {
                right[i] += 1;
                cur += 1;
            }
        }
    }

    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[i + 1] = cmp::max(dp[i + 1], dp[i]);
        if !ok[i] {
            continue;
        }
        dp[i + 2] = cmp::max(dp[i + 2], dp[i - 1] + 1);
        dp[i + 2] = cmp::max(dp[i + 2], dp[i - 1 - left[i]] + 1 + left[i]);
        dp[i + 2 + right[i]] = cmp::max(dp[i + 2 + right[i]], dp[i - 1] + 1 + right[i]);
        if left[i] > 0 {
            dp[i + 2] = cmp::max(dp[i + 2], dp[i - 1 - (left[i] - 1)] + 1 + (left[i] - 1));
        }
        if right[i] > 0 {
            dp[i + 2 + (right[i] - 1)] =
                cmp::max(dp[i + 2 + (right[i] - 1)], dp[i - 1] + 1 + (right[i] - 1));
        }
    }
    println!("{}", dp[n]);
}
