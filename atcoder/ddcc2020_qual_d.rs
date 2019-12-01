fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let m: usize = sc.read();
    let dc = (0..m)
        .map(|_| (sc.read::<u64>(), sc.read::<usize>()))
        .collect::<Vec<_>>();

    let mut res = vec![];
    for (d, c) in dc.into_iter() {
        let r = reduce(d, c);
        res.push(r);
    }
    let (_, ans) = reduce_v(res);
    println!("{:?}", ans - 1);
}

fn reduce(d: u64, mut c: usize) -> (u64, usize) {
    let mut res = vec![];
    let mut cur = d;
    let mut count = 1;
    while c > 0 {
        if c & 1 == 1 {
            res.push((cur, count));
        }
        cur = cur * 2;
        c >>= 1;
        count *= 2;
        if cur >= 10 {
            assert!(cur <= 18);
            cur = cur % 10 + cur / 10;
            count += 1;
        }
    }
    reduce_v(res)
}

fn reduce_v(res: Vec<(u64, usize)>) -> (u64, usize) {
    let mut cur = 0;
    let mut ans = 0;
    for (r, count) in res.into_iter() {
        cur += r;
        ans += count;
        if cur >= 10 {
            assert!(cur <= 18);
            cur = cur % 10 + cur / 10;
            ans += 1;
        }
    }
    (cur, ans)
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
