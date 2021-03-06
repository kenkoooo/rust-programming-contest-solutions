use std::collections::VecDeque;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let s = sc
        .read::<String>()
        .chars()
        .map(|c| c as usize - 'A' as usize)
        .collect::<Vec<_>>();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut removed = vec![false; n];
    let mut count = vec![vec![0; 2]; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        for &next in graph[i].iter() {
            count[i][s[next]] += 1;
        }
        if count[i][0] == 0 || count[i][1] == 0 {
            q.push_back(i);
        }
    }

    while let Some(remove) = q.pop_front() {
        removed[remove] = true;
        for &next in graph[remove].iter() {
            if removed[next] {
                continue;
            }
            count[next][s[remove]] -= 1;
            if count[next][s[remove]] == 0 {
                q.push_back(next);
            }
        }
    }

    if removed.contains(&false) {
        println!("Yes");
    } else {
        println!("No");
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
