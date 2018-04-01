use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let mut n: usize = sc.read();

    if n < 8 {
        match n {
            3 => println!("2 5 63"),
            4 => println!("2 3 4 9"),
            5 => println!("2 3 4 9 6"),
            6 => println!("2 3 4 9 6 12"),
            7 => println!("2 3 4 9 6 12 18"),
            _ => panic!(),
        }
        return;
    }

    let mut six = VecDeque::new();
    let mut odd = VecDeque::new();
    let mut even = VecDeque::new();

    for i in 2..30001 {
        if i % 6 == 0 {
            six.push_back(i);
        } else if i % 3 == 0 {
            odd.push_back(i);
        } else if i % 2 == 0 {
            even.push_back(i);
        }
    }


    let mut ans = Vec::new();
    while n >= 8 {
        ans.push(six.pop_front().unwrap());
        ans.push(six.pop_front().unwrap());
        ans.push(odd.pop_front().unwrap());
        ans.push(odd.pop_front().unwrap());
        ans.push(even.pop_front().unwrap());
        ans.push(even.pop_front().unwrap());
        ans.push(even.pop_front().unwrap());
        ans.push(even.pop_front().unwrap());
        n -= 8;
    }

    while n > 2 {
        if odd.len() >= 2 {
            ans.push(odd.pop_front().unwrap());
            ans.push(odd.pop_front().unwrap());
        } else {
            ans.push(even.pop_front().unwrap());
            ans.push(even.pop_front().unwrap());
        }
        n -= 2;
    }
    for _ in 0..n {
        ans.push(six.pop_front().unwrap());
    }

    for a in &ans {
        print!("{} ", *a);
    }
    println!();
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

