fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let max: i64 = *a.iter().max().unwrap();
    let min: i64 = *a.iter().min().unwrap();

    let negative_ans = |ans: &mut Vec<(usize, usize)>| {
        for i in (1..n).rev() {
            ans.push((i, i - 1));
        }
        println!("{}", ans.len());
        for &mut (from, to) in ans {
            println!("{} {}", from + 1, to + 1);
        }
    };
    let positive_ans = |ans: &mut Vec<(usize, usize)>| {
        for i in 1..n {
            ans.push((i - 1, i));
        }
        println!("{}", ans.len());
        for &mut (from, to) in ans {
            println!("{} {}", from + 1, to + 1);
        }
    };
    
    if max < 0 {
        let mut ans = Vec::new();
        negative_ans(&mut ans);
    } else if min >= 0 {
        let mut ans = Vec::new();
        positive_ans(&mut ans);
    } else {
        let mut ans = Vec::new();
        if max.abs() > min.abs() {
            let mut max_i = 0;
            for i in 0..n {
                if a[i] == max {
                    max_i = i;
                }
            }
            for i in 0..n {
                if i == max_i {
                    continue;
                }
                ans.push((max_i, i));
            }
            positive_ans(&mut ans);
        } else {
            let mut min_i = 0;
            for i in 0..n {
                if a[i] == min {
                    min_i = i;
                }
            }
            for i in 0..n {
                if i == min_i {
                    continue;
                }
                ans.push((min_i, i));
            }
            negative_ans(&mut ans);
        }
    }
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
