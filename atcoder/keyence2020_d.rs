use std::cmp;

const INF: usize = 1 << 50;
const V: usize = 51;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let b: Vec<usize> = sc.vec(n);
    let mut dp = vec![vec![INF; V]; 1 << n];
    dp[0][0] = 0;

    let mut masks = (0..(1 << n))
        .map(|mask: usize| (mask.count_ones() as usize, mask))
        .collect::<Vec<_>>();
    masks.sort();

    for (pos, cur_mask) in masks.into_iter() {
        for next_add in 0..n {
            if cur_mask & (1 << next_add) != 0 {
                continue;
            }
            let next_mask = cur_mask | (1 << next_add);

            let mut back_used = 0;
            for i in (next_add + 1)..n {
                if cur_mask & (1 << i) != 0 {
                    back_used += 1;
                }
            }
            let cur_pos = next_add + back_used;
            let distance = cur_pos - pos;

            let d = if pos > next_add {
                pos - next_add
            } else {
                next_add - pos
            };
            let card = if d % 2 == 0 { a[next_add] } else { b[next_add] };

            for cur_v in 0..V {
                let c = dp[cur_mask][cur_v];
                if c == INF {
                    continue;
                }
                if card >= cur_v {
                    dp[next_mask][card] = cmp::min(dp[next_mask][card], c + distance);
                }
            }
        }
    }
    let mut ans = INF;
    for i in 0..V {
        ans = cmp::min(ans, dp[(1 << n) - 1][i]);
    }

    if ans >= INF {
        println!("-1");
    } else {
        println!("{}", ans);
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
