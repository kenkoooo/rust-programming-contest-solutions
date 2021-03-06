fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let xor_sum = a.iter().fold(0, |xor, &a| xor ^ a);
    let mut a = a.into_iter().map(|a| a & !xor_sum).collect::<Vec<_>>();

    let mut rank = 0;
    for pos in (0..62).rev() {
        if let Some(i) = (rank..n).find(|&i| a[i] & (1 << pos) != 0) {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if a[j] & (1 << pos) != 0 {
                    a[j] ^= a[i];
                }
            }
            a.swap(i, rank);
            rank += 1;
        }
    }

    let max = a.into_iter().fold(0, |xor, a| xor ^ a);
    println!("{}", max * 2 + xor_sum);
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
