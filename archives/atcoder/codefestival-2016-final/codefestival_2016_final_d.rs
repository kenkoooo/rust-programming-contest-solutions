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
    input!(n: usize, m: usize, x: [usize; n]);

    let mut remainder = vec![vec![]; m];
    for &x in x.iter() {
        let r = x % m;
        remainder[r].push(x);
    }

    let mut ans = 0;
    for i in 0..m {
        if i == 0 || i * 2 == m {
            ans += remainder[i].len() / 2;
        } else if m - i < i {
            break;
        } else {
            let j = m - i;

            let len_i = remainder[i].len();
            let len_j = remainder[j].len();
            ans += cmp::min(len_i, len_j);

            let t = if len_i > len_j { i } else { j };
            remainder[t].sort();

            let mut pair = 0;
            let mut i = 1;
            while i < remainder[t].len() {
                if remainder[t][i - 1] == remainder[t][i] {
                    pair += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }

            let remain = cmp::max(len_i, len_j) - cmp::min(len_i, len_j);
            ans += cmp::min(remain / 2, pair);
        }
    }

    println!("{}", ans);
}
