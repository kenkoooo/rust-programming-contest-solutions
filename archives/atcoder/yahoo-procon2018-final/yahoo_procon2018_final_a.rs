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

fn get_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = vec![];
    for i in 2..(n + 1) {
        if is_prime[i] {
            primes.push(i);
            let mut pos = i * 2;
            while pos < is_prime.len() {
                is_prime[pos] = false;
                pos += i;
            }
        }
    }
    primes
}

use std::collections::BTreeSet;

fn main() {
    input!(n: usize, m: usize, a: [usize; n]);

    let primes = get_primes(350);
    let mut divisors = vec![vec![]; m + 1];
    let mut set = BTreeSet::new();
    for i in 1..(m + 1) {
        let mut a = i;
        for &prime in primes.iter() {
            if a % prime == 0 {
                divisors[i].push(prime);
                set.insert(prime);
                while a % prime == 0 {
                    a /= prime;
                }
            }
            if prime * prime > a {
                break;
            }
        }
        if a > 1 {
            divisors[i].push(a);
            set.insert(a);
        }
    }

    let max: usize = *a.iter().max().unwrap();
    let mut a_count = vec![0; max + 1];
    for &a in a.iter() {
        a_count[a] += 1;
    }

    let mut divide_count = vec![0; m + 1];
    for i in 2..(m + 1) {
        let mut cur = i;
        let mut count = 0;
        while cur <= max {
            count += a_count[cur];
            cur += i;
        }
        divide_count[i] = count;
    }

    for i in 1..(m + 1) {
        let divisors = &divisors[i];
        let n = divisors.len();
        let mut ans: i64 = 0;
        for mask in 1..(1 << n) {
            let mut count_ones = 0;
            let mut t = 1;
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    t *= divisors[i];
                    count_ones += 1;
                }
            }
            let sum = divide_count[t] as i64;
            let sum = if count_ones % 2 == 1 { sum } else { -sum };
            ans += sum;
        }
        println!("{}", a.len() as i64 - ans);
    }
}
