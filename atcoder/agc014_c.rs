use std::cmp;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    i: usize,
    j: usize,
    cost: usize,
    d: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.d.cmp(&self.d)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut sc = Scanner::new();
    let h = sc.usize_read();
    let w = sc.usize_read();
    let k = sc.usize_read();

    let a: Vec<Vec<char>> = (0..h)
        .map(|_| sc.read::<String>().chars().collect())
        .collect();
    let mut si = 0;
    let mut sj = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                si = i;
                sj = j;
            }
        }
    }

    let dij = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let inf = std::usize::MAX;
    let mut dist = vec![vec![inf; w]; h];
    dist[si][sj] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(State {
        i: si,
        j: sj,
        cost: 0,
        d: 0,
    });
    while let Some(State { i, j, cost, d }) = queue.pop() {
        if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
            println!("{}", (dist[i][j] + k - 1) / k);
            return;
        }
        for &(di, dj) in &dij {
            let ni = di + i as i32;
            let nj = dj + j as i32;
            if ni < 0 || nj < 0 {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;
            if ni >= h || nj >= w {
                continue;
            }

            let next_cost = if a[ni][nj] == '#' { 1 } else { 0 } + cost;
            let next_dist = dist[i][j] + 1;
            if dist[ni][nj] <= next_dist {
                continue;
            }

            if next_cost > 0 && (next_dist < k || next_cost > next_dist - k) {
                let next_dist = cmp::max((next_dist + k - 1) / k * k, next_cost + k);
                if dist[ni][nj] > next_dist {
                    dist[ni][nj] = next_dist;
                    queue.push(State {
                        i: ni,
                        j: nj,
                        cost: next_cost,
                        d: next_dist,
                    });
                }
            } else {
                dist[ni][nj] = next_dist;
                queue.push(State {
                    i: ni,
                    j: nj,
                    cost: next_cost,
                    d: next_dist,
                });
            }
        }
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
