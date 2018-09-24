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
    input!(n: usize, m: usize, a: [usize1; n]);
    let mut start = vec![vec![]; m];
    let mut end = vec![vec![]; m];
    for i in 0..(n - 1) {
        let from = a[i];
        let to = a[i + 1];
        start[from].push(to);
        end[to].push(from);
    }

    let mut total = 0;
    let mut count = 0;
    for from in 0..m {
        for &to in start[from].iter() {
            if from > to {
                count += 1;
                total += to + 1;
            } else {
                total += to - from;
            }
        }
    }
    let mut ans = total;
    for cur in 1..m {
        for &from in end[cur - 1].iter() {
            count -= 1;
            let to = cur - 1;
            total += (to + m - from) % m;
            total -= 1;
        }
        total -= count;

        ans = cmp::min(ans, total);
        count += start[cur - 1].len();
    }
    println!("{}", ans);
}
