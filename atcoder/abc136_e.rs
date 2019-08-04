fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: i64 = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let sum: i64 = a.iter().sum();
    let mut candidates = vec![];
    for i in 1.. {
        if sum < i * i {
            break;
        }
        if sum % i == 0 {
            candidates.push(i);
            if i * i != sum {
                candidates.push(sum / i);
            }
        }
    }
    candidates.sort();
    candidates.reverse();
    for &candidate in candidates.iter() {
        let mut add = vec![];
        let mut sum = 0;
        for &a in a.iter() {
            let upper = (a + candidate - 1) / candidate * candidate;
            sum += upper - a;
            add.push(upper - a);
        }
        add.sort();
        add.reverse();
        let mut negative = 0;
        if sum <= k && negative <= k {
            println!("{}", candidate);
            return;
        }
        for &a in add.iter() {
            sum -= a;
            negative += candidate - a;
            if sum <= k && negative <= k {
                println!("{}", candidate);
                return;
            }
        }
    }
    unreachable!()
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
