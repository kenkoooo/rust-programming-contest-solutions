use std::cmp;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);

    let mut start = vec![vec![]; m + 1];
    let mut end = vec![vec![]; m + 1];
    for i in 1..n {
        let from = a[i - 1];
        let to = a[i];
        start[from].push(to);
        end[to].push(from);
    }

    // initialize
    let mut total = 0;
    let mut segments = 0;
    for from in 0..(m + 1) {
        for &to in start[from].iter() {
            if from > to {
                segments += 1;
                total += to;
            } else {
                total += to - from;
            }
        }
    }

    let mut ans = total;
    for x in 1..m {
        for &from in end[x].iter() {
            segments -= 1;
            total -= 1;
            total += (x + m - from) % m;
        }
        total -= segments;
        ans = cmp::min(ans, total);
        segments += start[x].len();
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
