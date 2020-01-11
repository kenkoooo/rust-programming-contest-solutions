fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let x: Vec<u64> = sc.vec(n);
    let l: u64 = sc.read();

    let mut next = vec![vec![0; n]; 30];
    let mut tail = 0;
    for head in 0..n {
        while tail + 1 < n && x[tail + 1] <= l + x[head] {
            tail += 1;
        }
        next[0][head] = tail;
    }

    for b in 1..30 {
        for i in 0..n {
            next[b][i] = next[b - 1][next[b - 1][i]];
        }
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let (a, b) = if a > b { (b, a) } else { (a, b) };

        let mut cur = a;
        let mut ng = 0;
        for i in (0..30).rev() {
            let days = 1 << i;
            let next_pos = next[i][cur];
            if next_pos < b {
                cur = next_pos;
                ng += days;
            }
        }

        println!("{}", ng + 1);
    }
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
