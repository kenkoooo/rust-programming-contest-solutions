use std::cmp::min;
use std::collections::BinaryHeap;

const INF: i64 = 1 << 50;
fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let h: usize = sc.read();
    let w: usize = sc.read();
    let mut a = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            let v: i64 = sc.read();
            a[i][j] = v;
        }
    }

    let from_first = dijkstra(&a, (h - 1, 0));
    let from_second = dijkstra(&a, (h - 1, w - 1));
    let from_third = dijkstra(&a, (0, w - 1));

    let mut ans = INF;
    for i in 0..h {
        for j in 0..w {
            let first = from_first[i][j];
            let second = from_second[i][j];
            let third = from_third[i][j];

            let cost = first + second + third - 2 * a[i][j];
            ans = min(ans, cost);
        }
    }

    println!("{}", ans);
}

fn dijkstra(a: &Vec<Vec<i64>>, from: (usize, usize)) -> Vec<Vec<i64>> {
    let h = a.len();
    let w = a[0].len();
    let (i, j) = from;
    let mut from_first = vec![vec![INF; w]; h];
    from_first[i][j] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((-from_first[i][j], i, j));
    while let Some((_, i, j)) = heap.pop() {
        for &(di, dj) in [(0, -1), (0, 1), (1, 0), (-1, 0)].iter() {
            let ni = (i as i64) + di;
            let nj = (j as i64) + dj;
            if ni < 0 || nj < 0 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni >= h || nj >= w {
                continue;
            }

            if from_first[ni][nj] > from_first[i][j] + a[ni][nj] {
                from_first[ni][nj] = from_first[i][j] + a[ni][nj];
                heap.push((-from_first[ni][nj], ni, nj));
            }
        }
    }
    from_first
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
