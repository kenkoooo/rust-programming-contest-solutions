use std::collections::VecDeque;

const INF: u64 = 1e15 as u64;

const DX: [i64; 4] = [0, 0, 1, -1];
const DY: [i64; 4] = [1, -1, 0, 0];

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let k: u64 = sc.read();
    let s: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut dist = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                dist[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
            println!("1");
            return;
        }
        for d in 0..4 {
            let ni = (i as i64) + DX[d];
            let nj = (j as i64) + DY[d];
            if ni < 0 || nj < 0 || ni >= (h as i64) || nj >= (w as i64) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if dist[ni][nj] > dist[i][j] + 1 && s[ni][nj] != '#' && dist[i][j] + 1 <= k {
                dist[ni][nj] = dist[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if dist[i][j] != INF {
                dist[i][j] = k;
                q.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
            println!("{}", (dist[i][j] + k - 1) / k);
            return;
        }
        for d in 0..4 {
            let ni = (i as i64) + DX[d];
            let nj = (j as i64) + DY[d];
            if ni < 0 || nj < 0 || ni >= (h as i64) || nj >= (w as i64) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if dist[ni][nj] > dist[i][j] + 1 {
                dist[ni][nj] = dist[i][j] + 1;
                q.push_back((ni, nj));
            }
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
