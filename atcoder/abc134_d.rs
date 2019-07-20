fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let mut b = vec![0; n + 1];
    for i in (0..n).rev() {
        let a = a[i];
        let p = i + 1;
        let mut cur = p * 2;
        let mut sum = 0;
        while cur <= n {
            sum += b[cur];
            cur += p;
        }
        if sum % 2 == a {
            b[p] = 0;
        } else if (sum + 1) % 2 == a {
            b[p] = 1;
        } else {
            println!("-1");
            return;
        }
    }

    let mut ans = vec![];
    for i in 1..(n + 1) {
        if b[i] == 1 {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    for (i, ans) in ans.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans);
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
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
