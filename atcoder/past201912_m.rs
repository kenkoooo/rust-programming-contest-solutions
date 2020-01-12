fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut candidates = vec![];
    for _ in 0..n {
        let w: f64 = sc.read();
        let v: f64 = sc.read();
        candidates.push((w, v));
    }

    let mut ans = solve(&candidates);
    for _ in 0..m {
        let w: f64 = sc.read();
        let v: f64 = sc.read();
        candidates.push((w, v));
        let c = solve(&candidates);
        if c > ans {
            ans = c;
        }
        candidates.pop();
    }

    println!("{}", ans);
}

fn solve(candidates: &Vec<(f64, f64)>) -> f64 {
    let mut ok = 0.0;
    let mut ng = 1e19;
    let mut p_ok = ok;
    let mut p_ng = ng;
    loop {
        let x = (ng + ok) / 2.0;
        let mut c = vec![];
        for &(w, v) in candidates.iter() {
            let required = w * x;
            let diff = v - required;
            c.push((diff, w, v));
        }
        c.sort_by(|&(a, _, _), &(b, _, _)| a.partial_cmp(&b).unwrap());
        let mut sum_w = 0.0;
        let mut sum_v = 0.0;
        for &(_, w, v) in c.iter().rev().take(5) {
            sum_w += w;
            sum_v += v;
        }

        let a = sum_v / sum_w;
        if a > x {
            ok = x;
        } else {
            ng = x;
        }

        if ok == p_ok && ng == p_ng {
            break;
        }
        p_ok = ok;
        p_ng = ng;
    }
    ok
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
