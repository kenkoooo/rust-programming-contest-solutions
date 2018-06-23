const MAX_K: usize = 51;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let b: Vec<usize> = sc.read_vec(n);

    let mut can_use = vec![true; MAX_K];
    for i in (1..MAX_K).rev() {
        can_use[i] = false;

        let mut reachable = vec![vec![false; MAX_K]; MAX_K];
        for i in 0..MAX_K {
            reachable[i][i] = true;
        }

        for from in 0..MAX_K {
            for k in 1..MAX_K {
                if !can_use[k] {
                    continue;
                }
                let to = from % k;
                reachable[from][to] = true;
            }
        }

        for k in 0..MAX_K {
            for i in 0..MAX_K {
                for j in 0..MAX_K {
                    reachable[i][j] |= (reachable[i][k] && reachable[k][j]);
                }
            }
        }

        let mut ok = true;
        for i in 0..n {
            if !reachable[a[i]][b[i]] {
                ok = false;
                break;
            }
        }
        if !ok {
            can_use[i] = true;
        }

        if i == MAX_K - 1 && !ok {
            println!("-1");
            return;
        }
    }

    let mut ans: u64 = 0;
    for i in (0..MAX_K).rev() {
        ans *= 2;
        if i > 0 && can_use[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
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
