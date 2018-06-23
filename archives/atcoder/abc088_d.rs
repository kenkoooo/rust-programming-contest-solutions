use std::collections::VecDeque;


fn main() {
    let di: Vec<i32> = vec![0, 0, 1, -1];
    let dj: Vec<i32> = vec![1, -1, 0, 0];

    let mut sc = Scanner::new();
    let h: usize = sc.read();
    let w: usize = sc.read();
    let map: Vec<Vec<char>> = (0..h).map(|_| sc.read::<String>().chars().collect()).collect();

    let mut deque = VecDeque::new();
    let mut costs = vec![vec![std::usize::MAX; w]; h];

    deque.push_front((0, 0));
    costs[0][0] = 1;
    while let Some((i, j)) = deque.pop_front() {
        for d in 0..4 {
            let ni = i as i32 + di[d];
            let nj = j as i32 + dj[d];
            if ni < 0 || nj < 0 { continue; }

            let ni = ni as usize;
            let nj = nj as usize;
            if ni >= h || nj >= w { continue; }

            if map[ni][nj] == '#' { continue; }
            if costs[ni][nj] <= costs[i][j] + 1 { continue; }

            costs[ni][nj] = costs[i][j] + 1;
            deque.push_back((ni, nj));
        }
    }

    if costs[h - 1][w - 1] == std::usize::MAX {
        println!("-1");
        return;
    }
    let mut count = 0;
    for i in 0..h { for j in 0..w { if map[i][j] == '.' { count += 1; } } }

    println!("{}", count - costs[h - 1][w - 1]);
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

