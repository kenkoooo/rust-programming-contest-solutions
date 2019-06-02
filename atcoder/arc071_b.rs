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

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input!(n: usize, m: usize, x: [i64; n], y: [i64; m]);

    let sum = |x: Vec<i64>| -> Vec<i64> {
        let n = x.len();
        let mut sum_x = vec![0; n - 1];
        for i in 1..n {
            let segment = x[i] - x[i - 1];
            assert!(segment > 0);
            let left = i as i64;
            let right = (n - i) as i64;
            let sum = (left * right) % MOD;
            sum_x[i - 1] = (sum * segment) % MOD;
        }
        sum_x
    };

    let x = sum(x);
    let y = sum(y);
    let x: i64 = (x.iter().sum::<i64>()) % MOD;
    let y: i64 = (y.iter().sum::<i64>()) % MOD;
    println!("{}", (x * y) % MOD);
}
