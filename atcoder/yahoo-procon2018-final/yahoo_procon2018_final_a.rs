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
    input!(n: usize, m: usize, a: [usize; n]);

    let mut is_prime = vec![true; 320];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..is_prime.len() {
        if is_prime[i] {
            let mut cur = i * 2;
            while cur < is_prime.len() {
                is_prime[cur] = false;
                cur += i;
            }
        }
    }

    let mut primes = vec![];
    for i in 0..is_prime.len() {
        if is_prime[i] {
            primes.push(i);
        }
    }

    let mut count = vec![0; 100001];
    for &a in a.iter() {
        let mut a = a;
        let mut ps = vec![];
        for &p in primes.iter() {
            if a % p == 0 {
                ps.push(p);
                while a % p == 0 {
                    a /= p;
                }
            }
            if a == 1 {
                break;
            }
        }
        if a > 1 {
            ps.push(a);
        }

        let n = ps.len();
        for mask in 1..(1 << n) {
            let mut cur = 1;
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    cur *= ps[i];
                }
            }
            count[cur] += 1;
        }
    }

    for i in 1..(m + 1) {
        if i == 1 {
            println!("{}", n);
            continue;
        }

        let mut div = vec![];
        let mut cur = i;
        for &p in primes.iter() {
            if cur % p == 0 {
                div.push(p);
                while cur % p == 0 {
                    cur /= p;
                }
            }
            if cur == 1 {
                break;
            }
        }
        if cur > 1 {
            div.push(cur);
        }

        let mut ans = n as i64;

        let m = div.len();
        for mask in 1..(1 << m) {
            let mask: usize = mask;
            let mut cur = 1;
            for i in 0..m {
                if mask & (1 << i) != 0 {
                    cur *= div[i];
                }
            }
            if mask.count_ones() & 1 == 1 {
                ans -= count[cur] as i64;
            } else {
                ans += count[cur] as i64;
            }
        }

        println!("{}", ans);
    }
}
