fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n = sc.read();
    let mut a: Vec<usize> = sc.read_vec(n);
    a.sort();
    let mut used = vec![false; n];
    let mut ans = 0;
    for i in (0..31).rev() {
        let pow = 1 << i;
        let mut head = 0;
        let mut tail = n - 1;
        while head < tail {
            while head < tail && used[head] {
                head += 1;
            }
            while head < tail && (used[tail] || pow < a[head] + a[tail]) {
                tail -= 1;
            }
            if a[head] + a[tail] == pow && head < tail {
                used[head] = true;
                used[tail] = true;
                ans += 1;
            }
            head += 1;
        }
    }

    println!("{}", ans);
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
