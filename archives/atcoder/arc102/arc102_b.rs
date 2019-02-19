fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let mut l: usize = sc.read();

    let mut r = 0;
    while (1 << (r + 1)) <= l {
        r += 1;
    }

    let n = r + 1;
    let mut graph = vec![vec![]; n];
    for i in 0..r {
        graph[i].push((i + 1, 1 << i));
        graph[i].push((i + 1, 0));
    }

    let sum = (1 << r) - 1;
    for i in (0..(n - 1)).rev() {
        let to_i = (1 << i) - 1;
        if l >= to_i + 1 {
            let new_edge = l - to_i - 1;
            if new_edge > sum {
                // add paths [new_edge, new_edge+1, ..., l-1]
                graph[i].push((n - 1, new_edge));
                l = new_edge;
            }
        }
    }

    println!("{} {}", n, graph.iter().map(|e| e.len()).sum::<usize>());

    for (from, to, w) in graph
        .into_iter()
        .enumerate()
        .flat_map(|(i, e)| e.into_iter().map(move |(to, w)| (i, to, w)))
    {
        println!("{} {} {}", from + 1, to + 1, w);
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
