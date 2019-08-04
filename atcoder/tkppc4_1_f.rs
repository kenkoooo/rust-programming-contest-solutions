use std::cmp;

const INF: u32 = std::u32::MAX;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<Vec<u32>> = (0..n)
        .map(|_| (0..m).map(|_| sc.read::<u32>()).collect())
        .collect();
    let weight: Vec<Vec<u32>> = (0..n)
        .map(|_| (0..m).map(|_| sc.read::<u32>()).collect())
        .collect();

    let mut dist = vec![INF; n + 1];
    dist[0] = 0;
    for i in 0..n {
        let cur = dist[i];
        for j in 0..m {
            let a = a[i][j];
            let next = (cur + a - 1) / a * a + weight[i][j];
            dist[i + 1] = cmp::min(dist[i + 1], next);
        }
    }
    println!("{}", dist[n]);
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
