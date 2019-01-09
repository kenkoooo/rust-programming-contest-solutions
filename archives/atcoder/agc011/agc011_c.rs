fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n = sc.read();
    let m = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut pairs = vec![];
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            let mut count = 0;
            color[i] = 1;
            let is_bipartite = coloring(&graph, &mut color, i, &mut count);
            pairs.push((count, is_bipartite));
        }
    }

    let mut num_single = 0;
    let mut num_bipartite = 0;
    for &(count, is_bipartite) in pairs.iter() {
        if count == 1 {
            num_single += 1;
        } else if is_bipartite {
            num_bipartite += 1;
        }
    }
    let num_non_bipartite = pairs.len() - num_single - num_bipartite;
    let singles = num_single * n * 2 - num_single * num_single;
    let non_bipartites = num_non_bipartite * (num_bipartite + num_non_bipartite) * 2
        - num_non_bipartite * num_non_bipartite;
    let bipartites = num_bipartite * num_bipartite * 2;
    println!("{}", singles + non_bipartites + bipartites);
}

fn coloring(graph: &Vec<Vec<usize>>, color: &mut Vec<usize>, v: usize, count: &mut usize) -> bool {
    *count += 1;
    let mut result = true;
    for &next in graph[v].iter() {
        let next_color = if color[v] == 1 { 2 } else { 1 };
        if color[next] == 0 {
            color[next] = next_color;
            result &= coloring(graph, color, next, count);
        } else if color[next] != next_color {
            result = false;
        }
    }

    result
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
