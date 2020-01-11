use std::cmp::min;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: i64 = sc.read();
    let a: i64 = sc.read();
    let b: i64 = sc.read();

    let mut ans = b - 1;
    ans = min(ans, n - a);

    if (b - a) % 2 == 0 {
        ans = min((b - a) / 2, ans);
    }

    let next_a = a + (n - b) + 1;
    let next_b = n;
    if (next_b - next_a) % 2 == 0 {
        ans = min((next_b - next_a) / 2 + (n - b) + 1, ans);
    }

    let next_a = 1;
    let next_b = b - (a - 1) - 1;
    if (next_b - next_a) % 2 == 0 {
        ans = min((next_b - next_a) / 2 + a, ans);
    }

    println!("{}", ans);
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
