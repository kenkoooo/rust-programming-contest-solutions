fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    let mut inverse = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        inverse[b].push(a);
    }

    let probability = calc_probability(&graph, &inverse, (n, n));
    let mut ans = probability[0];
    for from in 0..n {
        if graph[from].len() == 1 {
            continue;
        }
        if let Some(&next) = graph[from]
            .iter()
            .max_by(|&&next1, &&next2| probability[next1].partial_cmp(&probability[next2]).unwrap())
        {
            let p = calc_probability(&graph, &inverse, (from, next));
            if p[0] < ans {
                ans = p[0];
            }
        }
    }

    println!("{}", ans);
}

fn calc_probability(
    graph: &Vec<Vec<usize>>,
    inverse: &Vec<Vec<usize>>,
    prohibited: (usize, usize),
) -> Vec<f64> {
    let n = inverse.len();
    let mut dp = vec![0.0; n];
    for next in (0..n).rev() {
        for &from in inverse[next].iter() {
            if (from, next) == prohibited {
                continue;
            }
            let s = if from == prohibited.0 {
                graph[from].len() as f64 - 1.0
            } else {
                graph[from].len() as f64
            };
            dp[from] += (dp[next] + 1.0) / s;
        }
    }
    //    println!("{:?} dp={:?}", prohibited, dp);
    dp
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
