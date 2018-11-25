extern crate num_bigint;
use num_bigint::BigInt;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();

    if n == 1 {
        println!("1");
        println!("+ 1");
        return;
    }

    let primes = get_primes(n);
    let primes: Vec<usize> = primes
        .iter()
        .map(|&p| {
            let mut cur = p;
            while cur * p <= n {
                cur *= p;
            }
            cur
        })
        .collect();
    let add_counts: Vec<usize> = primes
        .iter()
        .map(|&prime| {
            let mut other_lcm = 1;
            for &p in primes.iter() {
                if p == prime {
                    continue;
                }
                other_lcm = (other_lcm * p) % prime;
            }
            let mut cur = 0;
            let mut count = 0;
            while cur != 1 {
                cur = (cur + other_lcm) % prime;
                count += 1;
            }
            count
        })
        .collect();

    let mut numerator = BigInt::from(0);
    let mut denominator = BigInt::from(1);

    let mut ans = vec![];
    for i in 0..primes.len() {
        let prime = primes[i];
        let add_count = add_counts[i];

        let old_denominator = denominator.clone();
        denominator *= prime;
        numerator *= prime;

        if add_count * 2 > prime {
            let add_count = prime - add_count;
            numerator -= old_denominator * add_count;
            for _ in 0..add_count {
                ans.push((1, '-', prime));
            }
        } else {
            numerator += old_denominator * add_count;
            for _ in 0..add_count {
                ans.push((0, '+', prime));
            }
        }
    }

    let zero = BigInt::from(0);
    while numerator < zero {
        numerator += denominator.clone();
        ans.push((0, '+', 1));
    }
    while numerator > denominator.clone() {
        numerator -= denominator.clone();
        ans.push((1, '-', 1));
    }

    ans.sort();
    println!("{}", ans.len());
    for &(_, c, p) in ans.iter() {
        println!("{} {}", c, p);
    }
}

fn get_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 0..(n + 1) {
        if is_prime[p] {
            let mut cur = p * 2;
            while cur <= n {
                is_prime[cur] = false;
                cur += p;
            }
        }
    }

    (0..(n + 1)).filter(|&i| is_prime[i]).collect()
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
