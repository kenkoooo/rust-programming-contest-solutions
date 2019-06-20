const MOD: usize = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let comb = Combination::new(n + m + 1, MOD);
    let matrix = (0..n).map(|_| sc.vec(m)).collect();
    let rank = f2_gauss_jordan(matrix);

    let mut pow2 = vec![0; n + m + 1];
    pow2[0] = 1;
    for i in 0..(n + m) {
        pow2[i + 1] = (pow2[i] * 2) % MOD;
    }

    let mut ans = 0;
    for i in 1..(rank + 1) {
        let columns = comb.get(rank, i); // choose i columns
        let rows = pow2[i - 1]; // rows, which interact with selected columns
        let other_rows = pow2[rank - i];
        let outside_rows = pow2[n - rank];
        let outside_columns = pow2[m - rank];

        let mut tmp = (columns * rows) % MOD;
        tmp = (tmp * other_rows) % MOD;
        tmp = (tmp * outside_rows) % MOD;
        tmp = (tmp * outside_columns) % MOD;

        ans = (ans + tmp) % MOD;
    }
    println!("{}", ans);
}

pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Combination {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination {
            fact: fact,
            inv_fact: inv_fact,
            modulo: modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x >= y);
        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }

    pub fn h(&self, n: usize, r: usize) -> usize {
        self.get(n + r - 1, r)
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

fn f2_gauss_jordan(mut a: Vec<Vec<i64>>) -> usize {
    let n = a.len();
    let m = a[0].len();
    let mut rank = 0;
    let mut row = 0;
    for col in 0..m {
        match (row..n).filter(|&i| a[i][col] == 1).next() {
            Some(pivot) => {
                a.swap(row, pivot);
                rank += 1;
                for k in (row + 1)..n {
                    if a[k][col] == 1 {
                        for i in 0..m {
                            a[k][i] ^= a[row][i];
                        }
                    }
                }

                row += 1;
            }
            None => {}
        }

        if row >= n {
            break;
        }
    }
    rank
}
