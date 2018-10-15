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

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    a / g * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input!(n: usize, m: usize, s: chars, t: chars);

    let g = gcd(n, m);
    let x = n / g;
    let y = m / g;
    let a = lcm(x, y) / y;
    let b = lcm(x, y) / x;
    // l = i*lcm = i * n/gcd *m
    // a*l/n = i*y *a
    // b*l/m = i*x *b
    // a=lcm(x,y)/y

    for x in 0.. {
        let i = n / g * x;
        let j = m / g * x;
        if i >= s.len() || j >= t.len() {
            break;
        }
        if s[i] != t[j] {
            println!("-1");
            return;
        }
    }
    println!("{}", lcm(n, m));
}
