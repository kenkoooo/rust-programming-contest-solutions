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

use std::collections::BTreeSet;

fn main() {
    input!(s: chars, x: i64, y: i64);
    let mut v = vec![];
    let mut count = 0;
    for &c in s.iter() {
        if c == 'T' {
            v.push(count);
            count = 0;
        } else {
            count += 1;
        }
    }
    if count > 0 {
        v.push(count);
    }
    let x = x - v[0];

    let mut fx = vec![];
    let mut fy = vec![];
    for i in 1..v.len() {
        if i % 2 == 0 {
            fx.push(v[i]);
        } else {
            fy.push(v[i]);
        }
    }

    let x_sum = fx.iter().sum::<i64>();
    let y_sum = fy.iter().sum::<i64>();

    let mut xdp = BTreeSet::new();
    xdp.insert(0);
    for &fx in fx.iter() {
        let mut next = BTreeSet::new();
        for &cur in xdp.iter() {
            next.insert(cur + fx);
            next.insert(cur - fx);
        }
        xdp = next;
    }

    let mut ydp = BTreeSet::new();
    ydp.insert(0);
    for &fy in fy.iter() {
        let mut next = BTreeSet::new();
        for &cur in ydp.iter() {
            next.insert(cur + fy);
            next.insert(cur - fy);
        }
        ydp = next;
    }

    if xdp.contains(&x) && ydp.contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
