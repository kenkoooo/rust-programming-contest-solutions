use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut ans = sum[n];
    for suffix_head in 2..(n - 1) {
        let prefix_sum = sum[suffix_head];
        let suffix_sum = sum[n] - sum[suffix_head];

        let p = sum
            .binary_search_by_key(&(prefix_sum / 2 * 2 - 1), |&b| b * 2)
            .err()
            .unwrap();
        let s = sum
            .binary_search_by_key(&(suffix_sum / 2 * 2 + prefix_sum * 2 - 1), |&b| b * 2)
            .err()
            .unwrap();
        let p2_head_from = if p <= 4 { 1 } else { p - 4 };
        let p2_head_to = cmp::min(suffix_head, p + 4);

        let s2_head_from = cmp::max(if s >= 4 { s - 4 } else { 0 }, suffix_head + 1);
        let s2_head_to = cmp::min(n, s + 4);

        for p2_head in p2_head_from..p2_head_to {
            for s2_head in s2_head_from..s2_head_to {
                let p1_sum = sum[p2_head];
                let p2_sum = sum[suffix_head] - sum[p2_head];
                let s1_sum = sum[s2_head] - sum[suffix_head];
                let s2_sum = sum[n] - sum[s2_head];

                let sub_ans =
                    max4(p1_sum, p2_sum, s1_sum, s2_sum) - min4(p1_sum, p2_sum, s1_sum, s2_sum);
                ans = cmp::min(ans, sub_ans);
            }
        }
    }

    println!("{}", ans);
}

fn max4(x: usize, y: usize, z: usize, w: usize) -> usize {
    cmp::max(cmp::max(x, y), cmp::max(z, w))
}

fn min4(x: usize, y: usize, z: usize, w: usize) -> usize {
    cmp::min(cmp::min(x, y), cmp::min(z, w))
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
