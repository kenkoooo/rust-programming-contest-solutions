fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut a: Vec<(usize, usize)> = (0..n).map(|i| (sc.read::<usize>(), i)).collect();
    a.sort_by_key(|&(count, x)| (-(count as i32), x));

    let mut count = vec![0; n];

    let mut head = 0;
    loop {
        let (head_count, x) = a[head];
        let mut tail = head;
        while tail + 1 < n {
            let (_, y) = a[tail + 1];
            if y > x {
                tail += 1;
            } else {
                break;
            }
        }

        if tail == n - 1 {
            for i in head..n {
                let (c, _) = a[i];
                count[x] += c;
            }
            count[x] += head * head_count;
            break;
        }

        let (tail_count, _) = a[tail + 1];

        let mut tmp = 0;
        for i in head..(tail + 1) {
            let (c, _) = a[i];
            tmp += c - tail_count;
        }
        tmp += head * (head_count - tail_count);
        count[x] = tmp;
        head = tail + 1;
    }

    for i in 0..n {
        println!("{}", count[i]);
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
