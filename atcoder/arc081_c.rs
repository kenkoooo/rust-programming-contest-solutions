fn main() {
    let mut sc = Scanner::new();
    let a: Vec<usize> = sc.read::<String>()
        .chars()
        .map(|c| (c as usize) - ('a' as usize))
        .collect();
    let mut pos = vec![vec![]; 26];
    let n = a.len();
    for i in 0..n {
        pos[a[i]].push(i + 1);
    }

    let mut k = vec![0; n + 2];
    let mut vis = vec![false; 26];

    for i in (0..n).rev() {
        let j = i + 1;
        vis[a[i]] = true;

        let mut ok = true;
        for &b in &vis {
            ok = ok && b;
        }
        if ok {
            for i in 0..vis.len() {
                vis[i] = false;
            }
            k[j - 1] = k[j] + 1;
        } else {
            k[j - 1] = k[j];
        }
    }

    let mut cur_pos = 0;
    let mut ans: Vec<usize> = Vec::new();
    for _ in 0..(k[0] + 1) {
        for c in 0..26 {
            let p: usize = match pos[c].binary_search(&(cur_pos + 1)) {
                Ok(p) => pos[c][p],
                Err(p) => if p < pos[c].len() {
                    pos[c][p]
                } else {
                    n
                },
            };

            if p == n || k[cur_pos] == k[p] + 1 {
                cur_pos = p;
                ans.push(c);
                break;
            }
        }
    }

    for &a in &ans {
        let c = ((a as u8) + ('a' as u8)) as char;
        print!("{}", c);
    }
    println!();
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

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
