const EMPTY: usize = 2;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read::<usize>();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        let w = sc.read::<u64>();
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut color = vec![EMPTY; n];
    color[0] = 0;
    dfs(0, &graph, &mut color);
    for c in color.into_iter() {
        println!("{}", c);
    }
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, u64)>>, color: &mut Vec<usize>) {
    for &(next, weight) in graph[v].iter() {
        let next_color = if weight % 2 != 0 {
            color[v] ^ 1
        } else {
            color[v]
        };
        if color[next] != EMPTY {
            assert_eq!(color[next], next_color);
        } else {
            color[next] = next_color;
            dfs(next, graph, color);
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
