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
const INF: usize = MOD;

fn main() {
    input!(d: usize, f: usize, t: usize, n: usize, x: [usize; n]);
    let mut x = x;
    x.insert(0, 0);
    x.push(d);

    let n = x.len();

    let mut pow = vec![1; n + 1];
    for i in 0..n {
        pow[i + 1] = (pow[i] * 2) % MOD;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        if i > 0 {
            dp[i] += dp[i - 1];
            dp[i] %= MOD;
        }

        let from = x.upper_bound(x[i] + f - t);
        let to = x.upper_bound(x[i] + f);
        let skipped = from - i - 1;
        let combinations = (dp[i] * pow[skipped]) % MOD;

        if from < dp.len() {
            dp[from] += combinations;
            dp[from] %= MOD;
        }

        if to < dp.len() {
            dp[to] += MOD - combinations;
            dp[to] %= MOD;
        }
        if i == 0 {
            dp[i] = 0;
        }
    }

    dp[0] = 1;

    let mut ans = 0;
    for i in 0..(n - 1) {
        if d - x[i] <= f - t {
            ans += dp[i] * pow[n - 2 - i];
            ans %= MOD;
        }
    }

    ans += dp[n - 1];
    ans %= MOD;

    println!("{}", ans);
}

trait VecBound<T> {
    fn upper_bound(&self, key: T) -> usize;
}

impl VecBound<usize> for Vec<usize> {
    fn upper_bound(&self, key: usize) -> usize {
        self.binary_search_by_key(&(key * 2 + 1), |&v| v * 2)
            .err()
            .unwrap()
    }
}
