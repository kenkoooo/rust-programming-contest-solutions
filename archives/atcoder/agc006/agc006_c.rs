fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let x: Vec<i64> = sc.read_vec(n);
    let m: usize = sc.read();
    let k: usize = sc.read();
    let a: Vec<usize> = (0..m).map(|_| sc.read::<usize>() - 1).collect();

    let mut t: Vec<usize> = (0..(n - 1)).collect();
    for &a in a.iter() {
        t.swap(a, a - 1);
    }

    let result = pow(t, k);
    let mut ans: Vec<i64> = vec![0; n - 1];
    for i in 0..(n - 1) {
        ans[i] = x[result[i] + 1] - x[result[i]];
    }

    let mut cur = x[0];
    println!("{}", cur);
    for i in 0..(n - 1) {
        cur += ans[i];
        println!("{}", cur);
    }
}

fn pow(mut t: Vec<usize>, mut e: usize) -> Vec<usize> {
    let n = t.len();
    let mut result = (0..n).collect::<Vec<usize>>();
    while e > 0 {
        if e & 1 == 1 {
            result = (0..n).map(|i| result[t[i]]).collect();
        }
        e >>= 1;
        t = (0..n).map(|i| t[t[i]]).collect();
    }
    result
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
