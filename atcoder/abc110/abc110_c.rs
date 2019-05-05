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
use std::collections::BTreeMap;

fn main() {
    input!(s: chars, t: chars);
    let n = s.len();
    let s: Vec<usize> = s.iter().map(|&c| c as usize - 'a' as usize + 1).collect();
    let t: Vec<usize> = t.iter().map(|&c| c as usize - 'a' as usize + 1).collect();

    let mut to = vec![0; 27];
    for i in 0..n {
        if to[s[i]] == 0 {
            to[s[i]] = t[i];
        } else if to[s[i]] != t[i] {
            println!("No");
            return;
        }
    }

    let (s, t) = (t, s);

    let mut to = vec![0; 27];
    for i in 0..n {
        if to[s[i]] == 0 {
            to[s[i]] = t[i];
        } else if to[s[i]] != t[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
