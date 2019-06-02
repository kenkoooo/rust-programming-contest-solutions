fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut c: Vec<u64> = sc.vec(n);
    let sum = c.iter().sum::<u64>();
    c.sort();
    let mut ans = vec![0; n];
    ans[0] = c.pop().unwrap();
    dfs(0, 0, &mut c, &mut ans, &graph);
    println!("{}", sum - ans[0]);
    for (i, ans) in ans.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans);
    }
    println!();
}

fn dfs(v: usize, p: usize, c: &mut Vec<u64>, ans: &mut Vec<u64>, graph: &Vec<Vec<usize>>) {
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        let x = c.pop().unwrap();
        ans[next] = x;
        dfs(next, v, c, ans, graph);
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
