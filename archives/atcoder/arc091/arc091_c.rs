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
    input!(n: usize, a: usize, b: usize);
    if a + b > n + 1 {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    for i in 0..((n + b - 1) / b) {
        for j in 0..b {
            let tmp = (i + 1) * b - j;
            if tmp > n {
                continue;
            }
            ans.push(tmp);
        }
    }
    ans.resize(n, 0);
    let mut cur_a = (n + b - 1) / b;
    if cur_a > a {
        println!("-1");
        return;
    }
    for i in (0..((n + b - 1) / b)).rev() {
        let end = cmp::min(n, cmp::min((i + 1) * b, i * b + a - cur_a + 1));
        ans[(i * b)..end].sort();
        cur_a += (end - (i * b)) - 1;
    }
    assert_eq!(a, cur_a);
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
}
