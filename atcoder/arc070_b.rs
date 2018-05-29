fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut a: Vec<usize> = sc.read_vec(n);
    a.sort();

    let mut low = 0;
    let mut high = n;
    while high - low > 1 {
        let mid = (low + high) / 2;
        let check = is_needed(&a, mid, k);
        if is_needed(&a, mid, k) {
            high = mid;
        } else {
            low = mid;
        }
    }

    if is_needed(&a, 0, k) {
        println!("0");
    } else {
        println!("{}", low + 1);
    }
}

fn is_needed(a: &Vec<usize>, mid: usize, k: usize) -> bool {
    if a[mid] >= k {
        return true;
    }

    let mut ok = vec![false; k + 1];
    ok[0] = true;

    for i in 0..a.len() {
        if i == mid {
            continue;
        }
        for j in (0..k).rev() {
            if j + a[i] > k {
                continue;
            }
            if ok[j] {
                ok[j + a[i]] = true;
            }
        }
    }

    for i in 0..k {
        if ok[i] && i + a[mid] >= k {
            return true;
        }
    }
    return false;
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
