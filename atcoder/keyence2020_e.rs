use std::collections::VecDeque;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let d: Vec<u64> = sc.vec(n);
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push((b, i));
        graph[b].push((a, i));
    }

    let mut weights = vec![None; m];
    let mut tree = vec![vec![]; n];
    for i in 0..n {
        let mut candidates = vec![];
        if graph[i]
            .iter()
            .map(|&(next, _)| next)
            .all(|next| d[next] > d[i])
        {
            println!("-1");
            return;
        }

        for &(next, edge_id) in graph[i].iter() {
            candidates.push((d[next], next, edge_id));
        }
        candidates.sort();
        let (_, next, id) = candidates[0];
        tree[i].push(next);
        tree[next].push(i);
        match weights[id] {
            Some(weight) => {
                if weight != d[i] {
                    println!("-1");
                    return;
                }
            }
            None => {
                weights[id] = Some(d[i]);
            }
        }
    }

    let mut color = vec![None; n];
    for i in 0..n {
        if color[i].is_some() {
            continue;
        }
        color[i] = Some(true);
        let mut q = VecDeque::new();
        q.push_back(i);
        while let Some(v) = q.pop_front() {
            for &next in tree[v].iter() {
                match color[next] {
                    Some(_) => {
                        assert_ne!(color[next], color[v]);
                    }
                    None => {
                        color[next] = Some(!color[v].unwrap());
                        q.push_back(next);
                    }
                }
            }
        }
    }

    for i in 0..n {
        if color[i].unwrap() {
            sc.write("B");
        } else {
            sc.write("W");
        }
    }
    sc.write("\n");
    for i in 0..m {
        match weights[i] {
            Some(w) => sc.write(format!("{}\n", w)),
            None => sc.write("1000000000\n"),
        }
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
