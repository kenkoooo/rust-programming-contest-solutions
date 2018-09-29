fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let mut b: Vec<usize> = sc.read_vec(n);

    let mut ans = 0;
    for bit in (1..32).rev() {
        ans <<= 1;

        let mask = (1 << bit) - 1;
        let look = 1 << (bit - 1);
        b.sort_by_key(|&b| b & mask);

        let mut bit_count = 0;
        for &a in &a {
            let a = a & mask;

            let pos1 = b.binary_search_by_key(&(1 * look * 2 - 1), |&b| (a + (b & mask)) * 2)
                .err()
                .unwrap();
            let pos2 = b.binary_search_by_key(&(2 * look * 2 - 1), |&b| (a + (b & mask)) * 2)
                .err()
                .unwrap();
            let pos3 = b.binary_search_by_key(&(3 * look * 2 - 1), |&b| (a + (b & mask)) * 2)
                .err()
                .unwrap();

            let count = n - pos3 + pos2 - pos1;
            bit_count ^= count & 1;
        }
        ans += bit_count;
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
