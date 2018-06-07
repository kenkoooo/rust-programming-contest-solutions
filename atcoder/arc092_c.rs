use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);

    let mut odd = 0;
    let mut even = 0;
    for i in 0..n {
        if a[i] < 0 {
            continue;
        } else if i % 2 == 0 {
            even += a[i];
        } else {
            odd += a[i];
        }
    }

    let ans = cmp::max(odd, even);
    if ans == 0 {
        let mut max_pos = 0;
        for i in 0..n {
            if a[max_pos] < a[i] {
                max_pos = i;
            }
        }
        println!("{}", a[max_pos]);
        println!("{}", n - 1);
        for i in ((max_pos + 1)..n).rev() {
            println!("{}", i + 1);
        }
        for _ in 0..max_pos {
            println!("1");
        }
    } else {
        println!("{}", ans);
        let c = construct(&a, ans == even);
        println!("{}", c.len());
        for &c in &c {
            println!("{}", c + 1);
        }
    }
}

fn construct(a: &Vec<i64>, is_even: bool) -> Vec<usize> {
    let n = a.len();
    let mut indices = vec![];
    for i in 0..n {
        if is_even != (i % 2 == 0) {
            continue;
        }
        if a[i] < 0 {
            continue;
        }
        indices.push(i);
    }

    let m = indices.len();
    let mut ans = vec![];
    for i in ((indices[m - 1] + 1)..n).rev() {
        ans.push(i);
    }

    for i in (0..(m - 1)).rev() {
        let front = indices[i];
        let back = indices[i + 1];
        let d = back - front;
        for t in (1..(d / 2 + 1)).rev() {
            ans.push(front + t);
        }
    }

    for _ in 0..indices[0] {
        ans.push(0);
    }
    ans
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
