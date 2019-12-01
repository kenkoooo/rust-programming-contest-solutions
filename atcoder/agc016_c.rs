fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let h: usize = sc.read();
    let w: usize = sc.read();
    let r: usize = sc.read();
    let c: usize = sc.read();
    if h % r == 0 && w % c == 0 {
        sc.write("No\n".to_string());
    } else if h % r == 0 {
        sc.write("Yes\n".to_string());
        let ans = solve(w, c);
        for _ in 0..h {
            for i in 0..w {
                if i > 0 {
                    sc.write(" ".to_string());
                }
                sc.write(format!("{}", ans[i]));
            }
            sc.write("\n".to_string());
        }
    } else {
        sc.write("Yes\n".to_string());
        let ans = solve(h, r);
        for i in 0..h {
            for j in 0..w {
                if j > 0 {
                    sc.write(" ".to_string());
                }
                sc.write(format!("{}", ans[i]));
            }
            sc.write("\n".to_string());
        }
    }
}

fn solve(h: usize, r: usize) -> Vec<i64> {
    let mut s = vec![0; h + 1];
    for i in 0..(h + 1 - r) {
        s[i + r] = s[i] - 1;
    }
    let mut cur = h;
    s[h] = 1;
    loop {
        if cur < r {
            break;
        }
        s[cur - r] = s[cur] + 1;
        cur -= r;
    }
    let mut ans = vec![0; h];
    for i in 0..h {
        ans[i] = s[i + 1] - s[i];
    }
    ans
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
