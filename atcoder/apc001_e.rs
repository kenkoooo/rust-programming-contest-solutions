fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a: usize = sc.read();
        let b: usize = sc.read();
        graph[a].push(b);
        graph[b].push(a);
    }

    match (0..n).find(|&i| graph[i].len() >= 3) {
        Some(root) => {
            let (_, ans) = dfs(root, root, &graph);
            println!("{}", ans);
        }
        None => {
            println!("1");
        }
    }
}

fn dfs(v: usize, p: usize, graph: &Vec<Vec<usize>>) -> (bool, usize) {
    let children = graph[v]
        .iter()
        .filter(|&&next| next != p)
        .cloned()
        .collect::<Vec<_>>();
    if children.is_empty() {
        (true, 1)
    } else if children.len() == 1 {
        dfs(children[0], v, graph)
    } else {
        let mut sum = 0;
        let mut lines = false;
        for child in children.into_iter() {
            let (line, count) = dfs(child, v, graph);
            sum += count;
            lines |= line;
        }
        if lines {
            (false, sum - 1)
        } else {
            (false, sum)
        }
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
