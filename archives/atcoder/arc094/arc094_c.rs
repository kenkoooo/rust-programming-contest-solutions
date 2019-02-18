fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let ab = (0..n)
        .map(|_| (sc.read(), sc.read()))
        .collect::<Vec<(usize, usize)>>();

    if ab.iter().all(|&(a, b)| a == b) {
        println!("0");
    } else {
        let sum: usize = ab.iter().map(|&(_, b)| b).sum();
        let min_b = ab
            .iter()
            .filter(|&&(a, b)| a > b)
            .map(|&(_, b)| b)
            .min()
            .unwrap();
        println!("{}", sum - min_b);
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
