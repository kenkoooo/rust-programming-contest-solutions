fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut tree = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>();
        let b = sc.read::<usize>();
        tree[a].push(b);
        tree[b].push(a);
    }

    match (0..n).find(|&i| tree[i].len() > 2) {
        None => println!("1"),
        Some(root) => {
            let (_, ans) = dfs(root, root, &tree);
            println!("{}", ans);
        }
    }
}

fn dfs(v: usize, p: usize, graph: &Vec<Vec<usize>>) -> (bool, usize) {
    let mut children = 0;
    let mut sum = 0;
    let mut has_line = false;
    for &next in graph[v].iter().filter(|&&next| next != p) {
        children += 1;
        let (only_line, count) = dfs(next, v, graph);
        sum += count;
        assert!(!only_line || count == 1);
        has_line |= only_line;
    }

    if children == 0 {
        (true, 1)
    } else if children == 1 {
        (has_line, sum)
    } else {
        if has_line {
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
