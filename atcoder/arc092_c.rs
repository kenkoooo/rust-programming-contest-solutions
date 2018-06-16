fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let a: Vec<i64> = sc.read_vec(n);
    let max = a.iter().map(|&a| a).max().unwrap();
    let selected: Vec<usize> = if max < 0 {
        vec![(0..n).filter(|&i| a[i] == max).next().unwrap()]
    } else {
        let even: Vec<usize> = (0..n).filter(|&i| i % 2 == 0 && a[i] >= 0).collect();
        let odd: Vec<usize> = (0..n).filter(|&i| i % 2 != 0 && a[i] >= 0).collect();
        if even.iter().map(|&i| a[i]).sum::<i64>() > odd.iter().map(|&i| a[i]).sum() {
            even
        } else {
            odd
        }
    };

    println!("{}", selected.iter().map(|&i| a[i]).sum::<i64>());
    let mut procedure = vec![];
    for i in ((selected[selected.len() - 1] + 1)..n).rev() {
        procedure.push(i);
    }
    for i in (1..selected.len()).rev() {
        let head = selected[i - 1];
        let tail = selected[i];
        let num = (tail - head) / 2;
        for i in (0..num).rev() {
            procedure.push(head + i + 1);
        }
    }
    for _ in 0..selected[0] {
        procedure.push(0);
    }

    println!("{}", procedure.len());
    for &i in &procedure {
        println!("{}", i + 1);
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
