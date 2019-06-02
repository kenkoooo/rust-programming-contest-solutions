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

use std::collections::VecDeque;

fn main() {
    input!(n: usize, x: [usize1; n]);
    let mut ans = vec![0; n * n];
    for i in 0..n {
        ans[x[i]] = i + 1;
    }

    let mut q = VecDeque::new();
    for i in 0..(n * n) {
        if ans[i] == 0 {
            q.push_back(i);
        } else {
            if q.len() < ans[i] - 1 {
                println!("No");
                return;
            }
            for _ in 0..(ans[i] - 1) {
                let j = q.pop_front().unwrap();
                ans[j] = ans[i];
            }
        }
    }

    q.clear();
    for i in (0..(n * n)).rev() {
        if ans[i] == 0 {
            q.push_back(i);
        } else if x[ans[i] - 1] == i {
            let rest = n - ans[i];
            if q.len() < rest {
                println!("No");
                return;
            }
            for _ in 0..rest {
                let j = q.pop_front().unwrap();
                ans[j] = ans[i];
            }
        }
    }

    println!("Yes");
    for i in 0..(n * n) {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
}
