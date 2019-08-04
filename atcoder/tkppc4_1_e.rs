fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let k: usize = sc.read();
    let e: u64 = sc.read();
    let mut a: Vec<u64> = sc.vec(n);
    let mut b: Vec<u64> = sc.vec(m);
    a.sort();
    b.sort();
    b = b.into_iter().rev().take(k).collect();

    let a_sum: u64 = a.iter().sum();
    let b_sum: u64 = b.iter().sum();
    if e + b_sum >= a_sum {
        println!("Yes");
        let mut ans = 0;
        let mut e = e;
        for b in b.into_iter() {
            if e >= a_sum {
                break;
            }
            e += b;
            ans += 1;
        }
        println!("{}", ans);
    } else {
        println!("No");
        let mut sum = 0;
        let mut count = 0;
        for i in 0..n {
            if sum + a[i] > e + b_sum {
                break;
            }
            sum += a[i];
            count += 1;
        }
        println!("{}", count);
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
