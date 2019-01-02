fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let v: usize = sc.read();
    let e: usize = sc.read();
    let f: i64 = sc.read();

    let mut solver = primal_dual::MinimumCostFlowSolver::new(v);
    for _ in 0..e {
        let u: usize = sc.read();
        let v: usize = sc.read();
        let c: i64 = sc.read();
        let d: i64 = sc.read();
        solver.add_edge(u, v, c, d);
    }

    match solver.solve(0, v - 1, f) {
        Some(flow) => println!("{}", flow),
        None => println!("-1"),
    }
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

pub mod primal_dual {
    use std::cmp;
    use std::collections::BinaryHeap;
    use std::i64;
    type Flow = i64;
    type Cost = i64;
    const INF: Cost = i64::MAX >> 3;
    struct Edge {
        to: usize,
        capacity: Flow,
        flow: Flow,
        cost: Cost,
        reverse_to: usize,
        is_reversed: bool,
    }
    impl Edge {
        fn residue(&self) -> Flow {
            self.capacity - self.flow
        }
    }

    pub struct MinimumCostFlowSolver {
        graph: Vec<Vec<Edge>>,
    }

    impl MinimumCostFlowSolver {
        pub fn new(n: usize) -> Self {
            MinimumCostFlowSolver {
                graph: (0..n).map(|_| Vec::new()).collect(),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, capacity: Flow, cost: Cost) {
            let reverse_from = self.graph[to].len();
            let reverse_to = self.graph[from].len();
            self.graph[from].push(Edge {
                to: to,
                capacity: capacity,
                flow: 0,
                cost: cost,
                reverse_to: reverse_from,
                is_reversed: false,
            });
            self.graph[to].push(Edge {
                to: from,
                capacity: capacity,
                flow: capacity,
                cost: -cost,
                reverse_to: reverse_to,
                is_reversed: true,
            });
        }

        pub fn solve(&mut self, source: usize, sink: usize, mut flow: Flow) -> Option<Flow> {
            let n = self.graph.len();
            let mut result = 0;
            let mut h = vec![0; n];
            let mut prev_v: Vec<usize> = vec![0; n];
            let mut prev_e: Vec<usize> = vec![0; n];
            let mut q: BinaryHeap<(Cost, usize)> = BinaryHeap::new();
            while flow > 0 {
                let mut dist = vec![INF; n];
                dist[source] = 0;
                q.push((0, source));
                while let Some((cd, v)) = q.pop() {
                    if dist[v] < cd {
                        continue;
                    }
                    for (i, e) in self.graph[v].iter().enumerate() {
                        if e.residue() == 0 {
                            continue;
                        }
                        if dist[e.to] + h[e.to] > cd + h[v] + e.cost {
                            dist[e.to] = cd + h[v] + e.cost - h[e.to];
                            prev_v[e.to] = v;
                            prev_e[e.to] = i;
                            q.push((dist[e.to], e.to));
                        }
                    }
                }

                if dist[sink] == INF {
                    return None;
                }

                for i in 0..n {
                    h[i] += dist[i];
                }
                let mut df = flow;
                let mut v = sink;
                while v != source {
                    df = cmp::min(df, self.graph[prev_v[v]][prev_e[v]].residue());
                    v = prev_v[v];
                }
                flow -= df;
                result += df * h[sink];
                let mut v = sink;
                while v != source {
                    self.graph[prev_v[v]][prev_e[v]].flow += df;
                    let reversed_edge_id = self.graph[prev_v[v]][prev_e[v]].reverse_to;
                    self.graph[v][reversed_edge_id].flow -= df;
                    v = prev_v[v];
                }
            }
            Some(result)
        }
    }
}
