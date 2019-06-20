use num_bigint::BigInt;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    if n == 1 {
        println!("1");
        println!("+ 1");
        return;
    }
    let elements = (2..(n + 1))
        .filter(|&i| (2..i).all(|d| i % d != 0))
        .map(|prime| {
            let mut cur = prime;
            while cur * prime <= n {
                cur *= prime;
            }
            cur
        })
        .collect::<Vec<_>>();
    let counts = elements
        .iter()
        .map(|&e| {
            let mut rem = 1;
            for &element in elements.iter() {
                if element == e {
                    continue;
                }
                rem = (rem * element) % e;
            }
            (0..).find(|&count| (count * rem) % e == 1).unwrap()
        })
        .collect::<Vec<_>>();

    let mut add = vec![];
    let mut sub = vec![];
    let m = elements.len();
    for i in 0..m {
        let element = elements[i];
        let count = counts[i];
        if count * 2 > element {
            sub.push((element, element - count));
        } else {
            add.push((element, count));
        }
    }

    let mut p = BigInt::from(0);
    let mut q = BigInt::from(1);
    for &(e, c) in add.iter() {
        p *= e;
        q *= e;
        p += q.clone() / e * c;
    }
    for &(e, c) in sub.iter() {
        p *= e;
        q *= e;
        p -= q.clone() / e * c;
    }

    while p < BigInt::from(0) {
        p += q.clone();
        add.push((1, 1));
    }
    while p > BigInt::from(1) {
        p -= q.clone();
        sub.push((1, 1));
    }

    let mut ans = vec![];
    for (e, c) in add.into_iter() {
        for _ in 0..c {
            ans.push(('+', e));
        }
    }
    for (e, c) in sub.into_iter() {
        for _ in 0..c {
            ans.push(('-', e));
        }
    }

    println!("{}", ans.len());
    for (c, e) in ans.into_iter() {
        println!("{} {}", c, e);
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
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
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
