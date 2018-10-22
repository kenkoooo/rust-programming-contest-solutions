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

use std::collections::{BTreeSet, VecDeque};

const MOD: usize = 998244353;

fn brute_force(s: &Vec<char>) -> usize {
    let n = s.len();
    let mut result = BTreeSet::new();
    result.insert(s.clone());
    let mut q = VecDeque::new();
    q.push_back(s.clone());
    while let Some(s) = q.pop_front() {
        for i in 1..n {
            if s[i] != s[i - 1] {
                let mut s = s.clone();
                for c in 0..3 {
                    let c = ('a' as u8 + c as u8) as char;
                    if c != s[i] && c != s[i - 1] {
                        s[i] = c;
                        s[i - 1] = c;
                        break;
                    }
                }
                if !result.contains(&s) {
                    result.insert(s.clone());
                    q.push_back(s);
                }
            }
        }
    }
    result.len()
}

fn main() {
    input!(s: chars);
    let n = s.len();
    if n <= 10 {
        println!("{}", brute_force(&s));
        return;
    }
    if s.iter().all(|&c| c == s[0]) {
        println!("1");
        return;
    }

    let mut dp = vec![vec![vec![vec![0; 2]; 3]; 3]; n + 1];
    dp[0][0][0][0] = 1;

    for i in 0..n {
        for last_char in 0..3 {
            for remainder in 0..3 {
                for already in 0..2 {
                    for next in 0..3 {
                        let next_already = (already == 1) || (last_char == next && i > 0);
                        let next_already = if next_already { 1 } else { 0 };
                        let next_remainder = (remainder + next) % 3;
                        dp[i + 1][next][next_remainder][next_already] +=
                            dp[i][last_char][remainder][already];
                        dp[i + 1][next][next_remainder][next_already] %= MOD;
                    }
                }
            }
        }
    }

    let remainder = s.iter().map(|&c| c as usize - 'a' as usize).sum::<usize>() % 3;

    let mut ans = 0;
    for last in 0..3 {
        ans += dp[n][last][remainder][1];
    }

    let mut already = false;
    for i in 1..n {
        if s[i - 1] == s[i] {
            already = true;
        }
    }
    if !already {
        ans += 1;
    }
    ans %= MOD;

    println!("{}", ans);
}
