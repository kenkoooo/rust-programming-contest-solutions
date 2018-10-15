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

fn solve(mut a: Vec<i64>) -> usize {
    let n = a.len();
    let mut ans = 0;
    for i in 0..(n - 1) {
        if i % 2 == 0 && a[i] < a[i + 1] {
            if i + 2 < n && a[i + 1] > a[i + 2] && a[i] > a[i + 2] {
                a.swap(i + 1, i + 2);
            } else {
                a.swap(i, i + 1);
            }
            ans += 1;
        } else if i % 2 == 1 && a[i] > a[i + 1] {
            if i + 2 < n && a[i + 1] < a[i + 2] && a[i] < a[i + 2] {
                a.swap(i + 1, i + 2);
            } else {
                a.swap(i, i + 1);
            }
            ans += 1;
        }
    }
    ans
}

fn main() {
    input!(n: usize, a: [i64; n]);
    let b: Vec<i64> = a.iter().map(|&i| -i).collect();

    let a1 = solve(a);
    let a2 = solve(b);
    println!("{}", cmp::min(a1, a2));
}
