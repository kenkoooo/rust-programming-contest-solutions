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

fn main() {
    input!(n: usize, x: [f64; n], m: usize, k: usize, a: [usize1; m]);
    let mut swap: Vec<Vec<usize>> = vec![(0..(n - 1)).map(|i| i).collect()];
    let mut s = swap[0].clone();
    for &a in a.iter() {
        s.swap(a - 1, a);
    }
    swap.push(s);
    for i in 1..64 {
        let mut next = vec![0; n - 1];
        {
            let ref prev = &swap[i];
            for i in 0..(n - 1) {
                next[i] = prev[prev[i]];
            }
        }
        swap.push(next);
    }

    let mut operations: Vec<usize> = (0..(n - 1)).map(|i| i).collect();
    for i in 0..64 {
        if (1 << i) & k != 0 {
            let mut next = operations.clone();
            for j in 0..(n - 1) {
                next[j] = operations[swap[i + 1][j]];
            }
            operations = next;
        }
    }

    let num = operations
        .iter()
        .map(|&i| x[i + 1] - x[i])
        .collect::<Vec<_>>();
    let mut ans = vec![0.0; n];
    ans[0] = x[0];
    for i in 1..n {
        ans[i] = ans[i - 1] + num[i - 1];
    }
    for &ans in ans.iter() {
        println!("{}", ans);
    }
}
