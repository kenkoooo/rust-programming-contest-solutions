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

const INF: usize = 1e10 as usize;

fn main() {
    input!(n: usize, q: usize, a: [usize1; n], queries: [usize1; q]);

    let mut next = vec![INF; n];
    let mut prev = vec![INF; n];

    for i in 0..n {
        if i > 0 {
            prev[a[i]] = a[i - 1];
        }
        if i < n - 1 {
            next[a[i]] = a[i + 1];
        }
    }

    let mut start = a[0];
    let mut end = a[n - 1];

    for &q in queries.iter() {
        match (prev[q], next[q]) {
            (INF, INF) => {
                // do nothing
            }
            (INF, _) => {
                start = next[q];
                prev[start] = INF;

                prev[q] = end;
                next[q] = INF;

                next[end] = q;
                end = q;
            }
            (_, INF) => {
                end = prev[q];
                next[end] = INF;

                prev[q] = INF;
                next[q] = start;

                prev[start] = q;
                start = q;
            }
            _ => {
                let a = start;
                let b = prev[q];
                let c = next[q];
                let d = end;

                start = c;
                prev[c] = INF;

                prev[q] = d;
                next[d] = q;

                next[q] = a;
                prev[a] = q;

                next[b] = INF;
                end = b;
            }
        }
    }

    let mut cur = start;
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", cur + 1);
        cur = next[cur];
    }
    println!();
}
