use std::cmp;

fn solve(mut a: Vec<i64>) -> usize {
    let n = a.len();
    if n < 3 {
        return 0;
    }
    let mut ans = 0;
    for i in 0..(n - 2) {
        let mut t = vec![a[i], a[i + 1], a[i + 2]];
        if i % 2 == 1 {
            t = t.iter().map(|&a| -a).collect();
        }

        if t[0] > t[1] {
            continue;
        }
        if (t[0] < t[2] && t[2] < t[1]) || (t[0] < t[1] && t[1] < t[2]) {
            a.swap(i, i + 1);
            ans += 1;
        } else {
            a.swap(i + 1, i + 2);
            ans += 1;
        }
    }
    if (a[n - 3] > a[n - 2]) != (a[n - 2] < a[n - 1]) {
        a.swap(n - 2, n - 1);
        ans += 1;
    }
    ans
}

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let b: Vec<i64> = a.iter().map(|&a| -a).collect();
    println!("{}", cmp::min(solve(a), solve(b)));
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
