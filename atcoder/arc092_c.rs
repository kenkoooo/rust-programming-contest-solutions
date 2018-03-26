use std::cmp;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<i64> = (0..n).map(|_| sc.read()).collect();

    let mut odd = 0;
    let mut even = 0;
    for i in 0..n {
        if i % 2 == 0 {
            even += cmp::max(0, a[i]);
        } else {
            odd += cmp::max(0, a[i]);
        }
    }

    let mut all_negative = true;
    for i in 0..n {
        if a[i] >= 0 {
            all_negative = false;
        }
    }
    if all_negative {
        let mut max = 0;
        for i in 0..n {
            if a[max] < a[i] {
                max = i;
            }
        }

        println!("{}", a[max]);
        println!("{}", n - 1);
        let mut length = n;
        for i in 0..n {
            if i < max {
                println!("1");
                length -= 1;
            } else if i > max {
                println!("{}", length);
                length -= 1;
            }
        }
        return;
    }

    let mut ans = Vec::new();
    let mut q = VecDeque::from(a.clone());

    if odd > even {
        q.pop_front();
        ans.push(1);
    }

    loop {
        let head = q.pop_front().unwrap();
        q.push_front(head);
        if head >= 0 {
            break;
        } else {
            ans.push(1);
            ans.push(1);
            q.pop_front();
            q.pop_front();
        }
    }

    while q.len() >= 4 {
        let a1 = q.pop_front().unwrap();
        let a2 = q.pop_front().unwrap();
        let a3 = q.pop_front().unwrap();
        let a4 = q.pop_front().unwrap();
        if a3 >= 0 {
            ans.push(2);
            q.push_front(a4);
            q.push_front(a1 + a3);
        } else {
            ans.push(3);
            q.push_front(a2 + a4);
            q.push_front(a1);
        }
    }

    if q.len() == 3 {
        let back = q.pop_back().unwrap();
        if back > 0 {
            let a1 = q.pop_front().unwrap();
            q.pop_front();
            ans.push(2);
            q.push_front(a1 + back);
        } else {
            ans.push(3);
        }
    }

    if q.len() == 2 {
        q.pop_back();
        ans.push(2);
    }

    let max = q.pop_front().unwrap();
    println!("{}", max);
    assert!(q.is_empty());
    assert_eq!(max, cmp::max(odd, even));
    println!("{}", ans.len());

    let mut check = VecDeque::from(a.clone());
    for v in &ans {
        let v = *v;
        println!("{}", v);
        assert!(v <= 3);
        if v == 1 {
            check.pop_front();
        } else if v == 2 {
            if check.len() == 2 {
                check.pop_back();
            } else {
                let a1 = check.pop_front().unwrap();
                check.pop_front();
                let a2 = check.pop_front().unwrap();
                check.push_front(a1 + a2);
            }
        } else if v == 3 {
            if check.len() == 3 {
                check.pop_back();
            } else {
                let a1 = check.pop_front().unwrap();
                let a2 = check.pop_front().unwrap();
                check.pop_front().unwrap();
                let a4 = check.pop_front().unwrap();

                check.push_front(a2 + a4);
                check.push_front(a1);
            }
        }
    }

    assert!(check.len() <= 1);
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

