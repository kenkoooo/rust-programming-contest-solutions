use std::cmp;
use std::collections::BinaryHeap;
const INF: i64 = 1 << 50;
fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let w: i64 = sc.read();
    let c: i64 = sc.read();

    let mut rocks = vec![];
    for _ in 0..n {
        let l: i64 = sc.read();
        let r: i64 = sc.read();
        let cost: i64 = sc.read();
        rocks.push((l, r, cost));
    }
    rocks.push((w, INF, INF));
    rocks.sort();

    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut min = INF;
    let mut cur = 0;
    for (l, r, cost) in rocks.into_iter() {
        let head = l - c;
        if head >= 0 {
            while let Some((neg_right, cost)) = heap.pop() {
                let right = -neg_right;
                if right <= head {
                    cur -= cost;
                } else {
                    heap.push((neg_right, cost));
                    break;
                }
            }
            min = cmp::min(min, cur);
        }
        heap.push((-r, cost));
        cur += cost;
    }

    println!("{}", min);
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
