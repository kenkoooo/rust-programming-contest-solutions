use std::cmp;
use std::collections::VecDeque;

const INF: i32 = 1e9 as i32;
const DIR: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let k: i32 = sc.read();
    let board: Vec<_> = (0..h).map(|_| sc.chars()).collect();
    let mut dist = vec![vec![INF; w]; h];

    let mut si = 0;
    let mut sj = 0;
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 'S' {
                dist[i][j] = 0;
                si = i;
                sj = j;
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((0, 0, si, sj));
    for _ in 0.. {
        let mut next_q = VecDeque::new();
        while let Some((time, open, i, j)) = q.pop_front() {
            if i == h - 1 || j == w - 1 || i == 0 || j == 0 {
                let turn = (time + k - 1) / k;
                println!("{}", turn);
                return;
            }
            for (di, dj) in DIR.iter().cloned() {
                let ni = (i as i32) + di;
                let nj = (j as i32) + dj;
                if ni < 0 || nj < 0 {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if ni >= h || nj >= w {
                    continue;
                }

                if board[ni][nj] == '#' && open > 0 && dist[ni][nj] > time + 1 {
                    dist[ni][nj] = time + 1;
                    if (time + 1) % k == 0 {
                        next_q.push_back((time + 1, k, ni, nj));
                    } else {
                        q.push_back((time + 1, open - 1, ni, nj));
                    }
                } else if board[ni][nj] == '.' && dist[ni][nj] > time + 1 {
                    dist[ni][nj] = time + 1;
                    if (time + 1) % k == 0 {
                        next_q.push_back((time + 1, k, ni, nj));
                    } else {
                        q.push_back((time + 1, open, ni, nj));
                    }
                }
            }

            let next_time = (time + k) / k * k;
            next_q.push_front((next_time, k, i, j));
        }
        q = next_q;
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
