fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let h: usize = sc.read();
    let w: usize = sc.read();
    let k: usize = sc.read();
    let c: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect::<Vec<_>>();
    let mut ans = vec![vec![0; w]; h];
    let mut count = 1;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                ans[i][j] = count;
                count += 1;
            }
        }
    }
    assert_eq!(count - 1, k);

    for i in 0..h {
        for j in 1..w {
            if ans[i][j] == 0 && ans[i][j - 1] != 0 {
                ans[i][j] = ans[i][j - 1];
            }
        }
        for j in (0..(w - 1)).rev() {
            if ans[i][j] == 0 && ans[i][j + 1] != 0 {
                ans[i][j] = ans[i][j + 1];
            }
        }
    }
    for j in 0..w {
        for i in 1..h {
            if ans[i][j] == 0 && ans[i - 1][j] != 0 {
                ans[i][j] = ans[i - 1][j];
            }
        }
        for i in (0..(h - 1)).rev() {
            if ans[i][j] == 0 && ans[i + 1][j] != 0 {
                ans[i][j] = ans[i + 1][j];
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if j > 0 {
                sc.write(" ".to_string());
            }
            sc.write(format!("{}", ans[i][j]));
        }
        sc.write("\n".to_string());
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write(&mut self, s: String) {
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
