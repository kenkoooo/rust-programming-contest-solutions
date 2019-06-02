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

use std::collections::BTreeMap;

fn main() {
    input!(n: usize, s: chars);
    assert_eq!(s.len(), n * 2);

    let mut prefix_map: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut suffix_map: BTreeMap<(String, String), usize> = BTreeMap::new();
    for mask in 0..(1 << n) {
        let mut a = String::new();
        let mut b = String::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                a.push(s[i]);
            } else {
                b.push(s[i]);
            }
        }
        *prefix_map.entry((a, b)).or_insert(0) += 1;

        let mut a = String::new();
        let mut b = String::new();
        for i in (0..n).rev() {
            if mask & (1 << i) != 0 {
                a.push(s[i + n]);
            } else {
                b.push(s[i + n]);
            }
        }
        *suffix_map.entry((a, b)).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (key, &count) in prefix_map.iter() {
        if let Some(&a) = suffix_map.get(key) {
            ans += count * a;
        }
    }
    println!("{}", ans);
}
