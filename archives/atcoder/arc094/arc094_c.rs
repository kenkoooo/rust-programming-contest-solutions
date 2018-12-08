use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        a.push(sc.read::<usize>());
        b.push(sc.read::<usize>());
    }
    let sum = a.iter().sum::<usize>();
    let mut min = sum;
    for i in 0..n {
        if a[i] > b[i] {
            min = cmp::min(min, b[i]);
        }
    }
    if min == sum {
        println!("0");
    } else {
        println!("{}", sum - min);
    }
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
