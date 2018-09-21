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

fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = if a > b { (b, a) } else { (a, b) };
    if b % a == 0 {
        a
    } else {
        gcd(b % a, a)
    }
}

fn solve(init: i64, buy: i64, monitor: i64, add: i64) -> bool {
    // monitor < init + add * x - buy * y < buy
    if init < buy || add < buy {
        false
    } else if buy <= monitor {
        true
    } else {
        let g = gcd(add, buy);
        let low = monitor - init;
        let high = buy - init;
        if high <= 0 {
            let (high, low) = (-low, -high);
            let x = (high - 1) / g * g;
            if x > low {
                false
            } else {
                true
            }
        } else {
            let x = (high - 1) / g * g;
            if x > low {
                false
            } else {
                true
            }
        }
    }
}

fn main() {
    input!(t: usize, abcd: [(i64, i64, i64, i64); t]);

    for &(init, buy, monitor, add) in abcd.iter() {
        if solve(init, buy, monitor, add) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
