fn main() {
    let mut sc = Scanner::new();
    let mut get_sum = |sc: &mut Scanner| {
        let s: Vec<char> = sc.read::<String>().chars().collect();
        let n = s.len();
        let mut s_sum_a = vec![0; n + 1];
        let mut s_sum_b = vec![0; n + 1];
        for i in 0..n {
            s_sum_a[i + 1] = s_sum_a[i];
            s_sum_b[i + 1] = s_sum_b[i];
            if s[i] == 'A' {
                s_sum_a[i + 1] += 1;
            } else {
                s_sum_b[i + 1] += 1;
            }
        }
        (s_sum_a, s_sum_b)
    };
    let (s_sum_a, s_sum_b) = get_sum(&mut sc);
    let (t_sum_a, t_sum_b) = get_sum(&mut sc);

    let q: usize = sc.read();
    for _ in 0..q {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read::<usize>() - 1;
        let d = sc.read::<usize>() - 1;

        let s_a = (s_sum_a[b + 1] - s_sum_a[a]) % 3;
        let s_b = (s_sum_b[b + 1] - s_sum_b[a]) % 3;
        let t_a = (t_sum_a[d + 1] - t_sum_a[c]) % 3;
        let t_b = (t_sum_b[d + 1] - t_sum_b[c]) % 3;
        println!(
            "{}",
            if (s_a + 3 - s_b) % 3 == (t_a + 3 - t_b) % 3 {
                "YES"
            } else {
                "NO"
            }
        );
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
