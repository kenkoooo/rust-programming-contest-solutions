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
const INF: usize = 1 << 60;

fn main() {
    input!(n: usize, p: usize, x: [usize; n]);

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + x[i];
    }

    let mut min = INF;
    for k in 1..(n + 1) {
        let mut ans = p * k + n * p;
        for i in 0..n {
            if n < k * i {
                break;
            }
            let to = n - k * i;
            let from = if to >= k { to - k } else { 0 };

            let t = if i == 0 { 5 } else { 2 * i + 3 };
            ans += t * (sum[to] - sum[from]);
            if ans > INF {
                break;
            }
        }
        min = cmp::min(min, ans);
    }

    println!("{}", min);
}
