fn main() {
    let mut sc = Scanner::new();
    let a: Vec<usize> = sc
        .read::<String>()
        .chars()
        .map(|c| ((c as u8) - ('a' as u8)) as usize)
        .collect();
    let n = a.len();

    let mut used = vec![false; 26];
    let mut k = vec![0; n + 1];

    for i in (0..n).rev() {
        used[a[i]] = true;
        let mut ok = true;
        for &b in &used {
            ok &= b;
        }
        if ok {
            for i in 0..used.len() {
                used[i] = false;
            }
        }
        k[i] = if ok { k[i + 1] + 1 } else { k[i + 1] };
    }

    let mut indices = vec![vec![]; 26];
    for i in 0..n {
        indices[a[i]].push(i + 1);
    }

    let mut ans = Vec::new();
    let mut cur = 0;
    for _ in 0..(k[0] + 1) {
        for i in 0..26 {
            let t: usize = match indices[i].binary_search(&(cur + 1)) {
                Ok(t) => indices[i][t],
                Err(t) => if t == indices[i].len() {
                    n + 1
                } else {
                    indices[i][t]
                },
            };
            if t != n + 1 && k[cur] == k[t] {
                continue;
            }
            cur = t;
            ans.push(i);
            break;
        }
    }

    for &i in &ans {
        print!("{}", ((i as u8) + ('a' as u8)) as char);
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

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
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
