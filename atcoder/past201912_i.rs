use std::collections::BinaryHeap;

const INF: i64 = 1 << 50;
fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut edges = vec![];
    for _ in 0..m {
        let s = sc.chars();
        let mut mask = 0;
        for i in 0..n {
            if s[i] == 'Y' {
                mask |= 1 << i;
            }
        }

        let cost: i64 = sc.read();
        edges.push((mask, cost));
    }

    let mut dist = vec![INF; 1 << n];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((-dist[0], 0));
    while let Some((_, cur)) = heap.pop() {
        if cur == (1 << n) - 1 {
            println!("{}", dist[cur]);
            return;
        }
        for &(mask, cost) in edges.iter() {
            let next = mask | cur;
            if dist[next] > dist[cur] + cost {
                dist[next] = dist[cur] + cost;
                heap.push((-dist[next], next));
            }
        }
    }
    println!("-1");
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
