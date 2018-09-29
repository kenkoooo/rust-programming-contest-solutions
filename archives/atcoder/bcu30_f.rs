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
    input!(n: usize, a: [usize; n]);
    //dp[i+1]=sum(dp[x] + a[x]*...*a[i]*2^(x-1))
    //dp[i+1]=sum(dp[x]) + sum(a[x]*...*a[i]*2^(x-1))
    //sum(a[x]*...*a[i]*2^(x-1)) = sum(a[x]*..*a[i-1]*2^(x-1)) *a[i]+2^i

    let mut dp = vec![0; n + 1];
    dp[1] = a[0];
    let mut sum = 0;
    let mut mul = a[0];
    let mut pow2 = 1;
    for i in 1..n {
        sum += dp[i];
        sum %= MOD;
        mul *= a[i];
        mul += a[i] * pow2;
        mul %= MOD;
        dp[i + 1] = (sum + mul) % MOD;
        pow2 *= 2;
        pow2 %= MOD;
    }
    println!("{}", dp[n]);
}
