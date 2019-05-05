fn main() {
    let stdin = std::io::stdin();
    let mut sc = scanner::Scanner::new(stdin.lock());
    let n: usize = sc.read();
    let d1: i64 = sc.read();
    let d2: i64 = sc.read();

    let mut map = vec![vec![0; 2 * n]; 2 * n];
    let mut vertices = vec![];
    for i in 0..(2 * n) {
        for j in 0..(2 * n) {
            map[i][j] = vertices.len();
            vertices.push((i, j));
        }
    }

    let num_v = vertices.len();
    assert_eq!(num_v, 4 * n * n);
    let graph1 = construct_graph(&vertices, n, d1, &map);
    let mut color1 = vec![0; num_v];
    for i in 0..num_v {
        if color1[i] == 0 {
            color1[i] = 1;
            dfs(i, &graph1, &mut color1);
        }
    }

    let graph2 = construct_graph(&vertices, n, d2, &map);
    let mut color2 = vec![0; num_v];
    for i in 0..num_v {
        if color2[i] == 0 {
            color2[i] = 1;
            dfs(i, &graph2, &mut color2);
        }
    }

    let mut component = vec![vec![vec![]; 2]; 2];
    for i in 0..num_v {
        component[color1[i] - 1][color2[i] - 1].push(i);
    }

    for i in 0..2 {
        for j in 0..2 {
            if component[i][j].len() >= n * n {
                let ref component = component[i][j];
                for i in 0..(n * n) {
                    let (x, y) = vertices[component[i]];
                    println!("{} {}", x, y);
                }
                return;
            }
        }
    }
    unreachable!();
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<usize>) {
    for &next in graph[v].iter() {
        if color[next] == 0 {
            color[next] = if color[v] == 1 { 2 } else { 1 };
            dfs(next, graph, color);
        }
        assert!(color[next] != color[v]);
    }
}

fn construct_graph(
    vertices: &Vec<(usize, usize)>,
    n: usize,
    distance: i64,
    map: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let num_v = vertices.len();
    assert_eq!(num_v, 4 * n * n);
    let mut graph = vec![vec![]; num_v];
    for from in 0..num_v {
        let (i, j) = vertices[from];
        let i = i as i64;
        let j = j as i64;
        let n = n as i64;
        for x in 0..(2 * n) {
            let dx = i - x;
            let dy2 = distance - dx * dx;
            if dy2 < 0 {
                continue;
            }
            let dy = (dy2 as f64).sqrt() as i64;
            if dy * dy + dx * dx != distance {
                continue;
            }

            let y1 = dy + j;
            if y1 >= 0 && y1 < 2 * n {
                let to = map[x as usize][y1 as usize];
                graph[from].push(to);
            }

            let y2 = j - dy;
            if y2 >= 0 && y2 < 2 * n {
                let to = map[x as usize][y2 as usize];
                graph[from].push(to);
            }
        }
    }
    graph
}

pub mod scanner {
    use std::fmt::Debug;
    use std::io::Read;
    use std::str::{self, FromStr};

    pub struct Scanner<R: Read> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner {
                reader: reader,
                buf: Vec::new(),
            }
        }

        pub fn read<T>(&mut self) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            self.buf.clear();
            for c in self
                .reader
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n')
                .take_while(|&b| b != b' ' && b != b'\n')
            {
                self.buf.push(c);
            }

            unsafe { str::from_utf8_unchecked(&self.buf) }
                .parse()
                .expect("Parse error.")
        }
    }
}
