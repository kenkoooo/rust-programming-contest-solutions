fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let b: Vec<usize> = sc.vec(m);
    let max_a = *a.iter().max().unwrap();
    let max_b = *b.iter().max().unwrap();

    let mut pairs: Vec<Option<(usize, usize)>> = vec![None; max_a + max_b + 1];
    for i in 0..n {
        for j in 0..m {
            let sum = a[i] + b[j];
            match pairs[sum] {
                Some((x, y)) => {
                    println!("{} {} {} {}", x, y, i, j);
                    return;
                }
                None => {
                    pairs[sum] = Some((i, j));
                }
            }
        }
    }
    println!("-1");
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
