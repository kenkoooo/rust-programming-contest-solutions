use std::collections::VecDeque;

const INF: u64 = 1e15 as u64;
const D: [(i64, i64); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let a: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut dist = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                q.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        for &(di, dj) in D.iter() {
            let ni = di + (i as i64);
            let nj = dj + (j as i64);
            if ni < 0 || nj < 0 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni >= h || nj >= w {
                continue;
            }

            if dist[ni][nj] > dist[i][j] + 1 {
                dist[ni][nj] = dist[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }

    let max = dist
        .into_iter()
        .map(|d| d.into_iter().max().unwrap())
        .max()
        .unwrap();
    println!("{}", max);
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
