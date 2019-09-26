fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut a: Vec<u64> = sc.vec(n);
    let all = a.iter().fold(0, |xor, &a| xor ^ a);
    for i in 0..n {
        a[i] &= !all;
    }

    let mut rank = 0;
    for pos in (0..63).rev() {
        match (rank..n).find(|&i| a[i] & (1 << pos) != 0) {
            Some(x) => {
                a.swap(rank, x);
                for i in 0..n {
                    if i != rank && a[i] & (1 << pos) != 0 {
                        a[i] ^= a[rank];
                    }
                }
                rank += 1;
            }
            None => {}
        }
    }

    let max = a.iter().fold(0, |xor, &a| xor ^ a);
    println!("{}", max * 2 + all);
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
