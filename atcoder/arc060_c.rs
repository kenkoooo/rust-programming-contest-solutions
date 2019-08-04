const WIDTH: usize = 32;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let x: Vec<u64> = sc.vec(n);
    let l = sc.read();
    let mut go = vec![vec![0; n]; WIDTH];

    let mut tail = 0;
    for head in 0..n {
        while tail < n && (head > tail || x[tail] - x[head] <= l) {
            tail += 1;
        }
        go[0][head] = tail - 1;
    }

    for t in 1..WIDTH {
        for i in 0..n {
            go[t][i] = go[t - 1][go[t - 1][i]];
        }
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let (mut a, b) = if a > b { (b, a) } else { (a, b) };
        let mut ng = 0;
        for i in (0..WIDTH).rev() {
            let cost = 1 << i;
            if go[i][a] < b {
                ng += cost;
                a = go[i][a];
            }
        }
        println!("{}", ng + 1);
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
