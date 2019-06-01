use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let m: usize = sc.read();
    let k: usize = sc.read();
    let max = (1 << m) - 1;
    if k > max {
        println!("-1");
        return;
    }

    if m == 1 {
        if k == 0 {
            println!("0 0 1 1");
        } else {
            println!("-1");
        }
        return;
    }

    let mut ans = VecDeque::new();
    ans.push_back(k);
    for i in 0..(max + 1) {
        if i == k {
            continue;
        }
        ans.push_front(i);
        ans.push_back(i);
    }
    ans.push_front(k);

    for a in ans.into_iter() {
        print!("{} ", a);
    }
    println!();
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
