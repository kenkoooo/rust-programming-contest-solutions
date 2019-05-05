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

fn rev(x: usize) -> usize {
    x.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

use std::cmp;
use std::collections::VecDeque;

const MAX_N: usize = 1000;

fn main() {
    input!(n: usize, m: usize);
    let (n, m) = (cmp::max(n, m), cmp::min(n, m));

    let mut dp = vec![vec![false; MAX_N]; MAX_N];
    let mut queue = VecDeque::new();
    for i in 1..MAX_N {
        queue.push_back((0, i));
        dp[i][0] = true;
        queue.push_back((i, 0));
        dp[0][i] = true;
    }
    dp[0][0] = true;

    let mut vis = vec![vec![false; MAX_N]; MAX_N];

    while let Some((x, y)) = queue.pop_front() {
        if vis[x][y] {
            continue;
        }
        vis[x][y] = true;

        if y + x > x {
            let x = x;
            let y = y + x;
            if y < MAX_N && x > 0 && x % 10 != 0 {
                let mut x = rev(x);
                while x < y {
                    if x < MAX_N && !dp[x][y] {
                        dp[x][y] = true;
                        queue.push_back((x, y));
                    }
                    x *= 10;
                }
            }
            if x < MAX_N && y > 0 && y % 10 != 0 {
                let mut y = rev(y);
                while x >= y {
                    if y < MAX_N && !dp[x][y] {
                        dp[x][y] = true;
                        queue.push_back((x, y));
                    }
                    y *= 10;
                }
            }
        }
        if x + y >= y {
            let x = x + y;
            let y = y;
            if y < MAX_N && x > 0 && x % 10 != 0 {
                let mut x = rev(x);
                while x < y {
                    if x < MAX_N && !dp[x][y] {
                        dp[x][y] = true;
                        queue.push_back((x, y));
                    }
                    x *= 10;
                }
            }
            if x < MAX_N && y > 0 && y % 10 != 0 {
                let mut y = rev(y);
                while x >= y {
                    if y < MAX_N && !dp[x][y] {
                        dp[x][y] = true;
                        queue.push_back((x, y));
                    }
                    y *= 10;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 1..(n + 1) {
        for j in 1..(m + 1) {
            if !dp[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
