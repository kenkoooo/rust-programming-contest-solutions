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
extern crate num_bigint;
use num_bigint::BigInt;

fn solve(n: usize) -> Vec<(usize, char, usize)> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..(n + 1) {
        if is_prime[i] {
            let mut cur = i * 2;
            while cur <= n {
                is_prime[cur] = false;
                cur += i;
            }
        }
    }

    let primes = (0..(n + 1))
        .filter(|&i| is_prime[i])
        .map(|p| {
            let mut cur = p;
            while cur * p <= n {
                cur *= p;
            }
            cur
        }).collect::<Vec<_>>();
    let addition = primes
        .iter()
        .map(|&prime| {
            let mut cur = 1;
            for &p in primes.iter() {
                if p == prime {
                    continue;
                }
                cur = (cur * p) % prime;
            }
            let add = cur;
            let mut cur = 0;
            let mut count = 0;
            while cur != 1 {
                cur = (cur + add) % prime;
                count += 1;
            }
            count
        }).collect::<Vec<_>>();

    let mut ans = vec![];

    let mut numerator = BigInt::from(0);
    let mut denominator = BigInt::from(1);

    let mut count = 0;
    for i in 0..primes.len() {
        let prime = primes[i];
        let add = addition[i];
        assert!(prime > add);

        let old_denominator = denominator.clone();
        denominator *= prime;
        numerator *= prime;

        if add * 2 > prime {
            let add = prime - add;
            numerator -= old_denominator * add;
            count += add;
            for _ in 0..add {
                ans.push((1, '-', prime));
            }
        } else {
            numerator += old_denominator * add;
            count += add;
            for _ in 0..add {
                ans.push((0, '+', prime));
            }
        }
    }

    let zero = BigInt::from(0);

    while numerator < zero {
        numerator += denominator.clone();
        count += 1;
        ans.push((0, '+', 1));
    }
    while numerator > denominator.clone() {
        numerator -= denominator.clone();
        count += 1;
        ans.push((1, '-', 1));
    }
    assert_eq!(count, ans.len());
    ans.sort();
    ans
}

fn main() {
    input!(n: usize);
    if n == 1 {
        println!("1");
        println!("+ 1");
        return;
    }
    let ans = solve(n);
    println!("{}", ans.len());
    for &(_, c, p) in ans.iter() {
        println!("{} {}", c, p);
    }
}
