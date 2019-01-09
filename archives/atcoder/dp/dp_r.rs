const MOD: usize = 1e9 as usize + 7;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };

    let n = sc.read();
    let k: usize = sc.read();

    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            graph[i][j] = sc.read();
        }
    }

    let mut k = k;
    let mut result = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1 } else { 0 }).collect())
        .collect();
    let mut cur = graph;
    while k > 0 {
        if (k & 1) != 0 {
            result = matrix_mul(&result, &cur);
        }
        cur = matrix_mul(&cur, &cur);
        k >>= 1;
    }

    let v = mul(vec![1; n], &result);
    let ans: usize = v.iter().sum();
    println!("{}", ans % MOD);
}

fn matrix_mul(m1: &Vec<Vec<usize>>, m2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = m1.len();
    let mut result = vec![vec![0; n]; n];
    for i1 in 0..n {
        for i2 in 0..n {
            for k in 0..n {
                result[i1][i2] += m1[i1][k] * m2[k][i2];
                result[i1][i2] %= MOD;
            }
        }
    }
    result
}

fn mul(v: Vec<usize>, m: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = v.len();
    let mut result = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            result[j] += v[i] * m[i][j];
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
