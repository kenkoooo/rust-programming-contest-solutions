use std::collections::VecDeque;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queries = vec![];
    let q: usize = sc.read();
    for _ in 0..q {
        let v = sc.read::<usize>() - 1;
        let d: usize = sc.read();
        let color: usize = sc.read();
        queries.push((v, d, color));
    }

    let mut height = vec![0; n];
    let mut v_color = vec![0; n];
    let mut q = VecDeque::new();
    for (start, dist, color) in queries.into_iter().rev() {
        if v_color[start] == 0 {
            v_color[start] = color;
        }
        if height[start] >= dist {
            continue;
        }
        q.push_back((start, dist, color));
        while let Some((v, dist, color)) = q.pop_front() {
            for &next in graph[v].iter() {
                if v_color[next] == 0 {
                    v_color[next] = color;
                }
                if height[next] >= dist - 1 {
                    continue;
                }
                height[next] = dist - 1;
                q.push_back((next, dist - 1, color));
            }
        }
    }
    for c in v_color.into_iter() {
        sc.write(format!("{}\n", c));
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
