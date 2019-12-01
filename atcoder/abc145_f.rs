use std::cmp::min;
use std::collections::BTreeMap;

const INF: i64 = 1e17 as i64;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let k: usize = sc.read();
    let h: Vec<i64> = sc.vec(n);

    let mut dp = BTreeMap::new();
    dp.insert((0, 0), 0);
    for height in h.into_iter() {
        let mut next = BTreeMap::new();
        for ((cur_height, cur_ops), min_cost) in dp.into_iter() {
            if cur_height >= height {
                {
                    let cur = next.entry((height, cur_ops)).or_insert(INF);
                    update_min(cur, min_cost);
                }
                if cur_ops < k {
                    let cur = next.entry((cur_height, cur_ops + 1)).or_insert(INF);
                    update_min(cur, min_cost);
                }
            } else {
                {
                    let cur = next.entry((height, cur_ops)).or_insert(INF);
                    update_min(cur, min_cost + height - cur_height);
                }
                if cur_ops < k {
                    let cur = next.entry((cur_height, cur_ops + 1)).or_insert(INF);
                    update_min(cur, min_cost);
                }
            }
        }
        dp = next;
    }

    println!("{}", dp.values().cloned().min().unwrap());
}

fn update_min<T: PartialOrd>(cur: &mut T, v: T) {
    if *cur > v {
        *cur = v;
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
