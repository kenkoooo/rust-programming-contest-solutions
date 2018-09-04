use std::cmp;
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

const INF: usize = 1e9 as usize;
fn main() {
    input!(
        n: usize,
        goal: usize,
        initial: usize,
        ab: [(usize, usize); n],
    );

    let mut dp = vec![vec![INF; goal + 1]; 1 << n];
    dp[0][0] = 0;
    for mask in 0..(1 << n) {
        for cookie in 0..(goal + 1) {
            let mut power = initial;
            for i in 0..n {
                if (1 << i) & mask != 0 {
                    power += ab[i].1;
                }
            }
            let next_cookie = cmp::min(goal, cookie + power);
            dp[mask][next_cookie] = cmp::min(dp[mask][next_cookie], dp[mask][cookie] + 1);
            for i in 0..n {
                if (1 << i) & mask != 0 {
                    continue;
                }
                if cookie < ab[i].0 {
                    continue;
                }
                let next_mask = (1 << i) | mask;
                let next_cookie = cookie - ab[i].0;
                dp[next_mask][next_cookie] = cmp::min(dp[next_mask][next_cookie], dp[mask][cookie]);
            }
        }
    }

    let mut ans = INF;
    for mask in 0..(1 << n) {
        ans = cmp::min(ans, dp[mask][goal]);
    }
    println!("{}", ans);
}
