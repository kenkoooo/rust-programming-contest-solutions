use std::cmp;

const INF: i64 = 1e18 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let l: i64 = sc.read();
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read();
        dist[a][b] = c;
        dist[b][a] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut fill = vec![vec![n; n]; n];
    for i in 0..n {
        fill[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                fill[i][j] = 0;
            } else if dist[i][j] <= l {
                fill[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                fill[i][j] = cmp::min(fill[i][j], fill[i][k] + fill[k][j]);
            }
        }
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let from = sc.read::<usize>() - 1;
        let to = sc.read::<usize>() - 1;
        let ans = fill[from][to];
        if ans == n {
            println!("-1");
        } else {
            println!("{}", ans - 1);
        }
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
