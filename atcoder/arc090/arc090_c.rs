use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::usize;

const MOD: usize = 1000000007;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (n, m) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let mut graph = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
    let (s, t) = {
        let v = read_values::<usize>();
        (v[0] - 1, v[1] - 1)
    };

    for _ in 0..m {
        let v = read_values::<usize>();
        let from = v[0] - 1;
        let to = v[1] - 1;
        let d = v[2];
        graph[from].push((to, d));
        graph[to].push((from, d));
    }

    let (count_s, dist_s) = first_dijkstra(&graph, s, t);
    let (count_t, dist_t) = first_dijkstra(&graph, t, s);

    let shortest_dist = dist_s[t];

    let mut edge_set = BTreeSet::new();
    for from in 0..n {
        for e in &graph[from] {
            let (to, c) = *e;

            if dist_s[from] + c + dist_t[to] == shortest_dist
                && dist_s[from] * 2 < shortest_dist
                && dist_t[to] * 2 < shortest_dist {
                edge_set.insert((from, to));
            }
        }
    }

    let mut ans = count_s[t] * count_t[s];
    ans %= MOD;
    for e in &edge_set {
        let (from, to) = *e;
        let (from, to) = if dist_s[from] < dist_s[to] {
            (from, to)
        } else {
            (to, from)
        };

        let mut c = (count_s[from] * count_s[from]) % MOD;
        c = (c * count_t[to]) % MOD;
        c = (c * count_t[to]) % MOD;
        ans = (ans + MOD - c) % MOD;
    }

    for v in 0..n {
        if dist_s[v] * 2 == shortest_dist {
            let mut c = (count_s[v] * count_s[v]) % MOD;
            c = (c * count_t[v]) % MOD;
            c = (c * count_t[v]) % MOD;
            ans = (ans + MOD - c) % MOD;
        }
    }
    println!("{}", ans);
}

fn first_dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize, t: usize) -> (Vec<usize>, Vec<usize>) {
    let n = graph.len();
    let mut count = vec![0; n];
    count[s] = 1;
    let mut dist = vec![usize::MAX; n];
    dist[s] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, position: s });

    let mut vis = vec![false; n];

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[t] {
            break;
        }

        if vis[position] {
            continue;
        }
        vis[position] = true;

        for e in &graph[position] {
            let (to, d) = *e;
            if dist[to] < dist[position] + d {
                continue;
            }
            if dist[to] > dist[position] + d {
                dist[to] = dist[position] + d;
                count[to] = 0;
                heap.push(State { cost: dist[to], position: to });
            }

            if dist[to] == dist[position] + d {
                count[to] += count[position];
                count[to] %= MOD;
            }
        }
    }
    (count, dist)
}

fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}
