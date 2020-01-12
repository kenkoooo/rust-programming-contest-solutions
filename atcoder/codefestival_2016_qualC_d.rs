use std::cmp;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let h: usize = sc.read();
    let w: usize = sc.read();
    let board: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();

    let mut ans = 0;
    for j in 1..w {
        let a = (0..h).map(|i| board[h - 1 - i][j - 1]).collect();
        let b = (0..h).map(|i| board[h - 1 - i][j]).collect();
        let cost = calc(&a, &b);
        ans += cost;
    }
    println!("{}", ans);
}

fn calc(a: &Vec<char>, b: &Vec<char>) -> u64 {
    let h = a.len();
    let mut cost = vec![vec![0; h + 1]; h + 1];
    for broken_a in 0..(h + 1) {
        let mut c = 0;
        for i in 0..h {
            if i + broken_a >= h {
                break;
            }
            if a[i + broken_a] == b[i] {
                c += 1;
            }
        }
        cost[broken_a][0] = c;
        for i in 0..h {
            if broken_a + i >= h {
                break;
            }
            cost[broken_a + i + 1][i + 1] =
                cost[broken_a + i][i] - if a[broken_a + i] == b[i] { 1 } else { 0 };
        }
    }
    for broken_b in 0..(h + 1) {
        let mut c = 0;
        for i in 0..h {
            if i + broken_b >= h {
                break;
            }
            if a[i] == b[i + broken_b] {
                c += 1;
            }
        }
        cost[0][broken_b] = c;
        for i in 0..h {
            if broken_b + i >= h {
                break;
            }
            cost[i + 1][broken_b + i + 1] =
                cost[i][broken_b + i] - if a[i] == b[broken_b + i] { 1 } else { 0 };
        }
    }

    let mut dp = vec![vec![1 << 50; h + 1]; h + 1];
    dp[0][0] = 0;
    for i in 0..(h + 1) {
        for j in 0..(h + 1) {
            if i < h {
                dp[i + 1][j] = cmp::min(dp[i + 1][j], dp[i][j] + cost[i][j]);
            }
            if j < h {
                dp[i][j + 1] = cmp::min(dp[i][j + 1], dp[i][j] + cost[i][j]);
            }
        }
    }
    dp[h][h]
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
