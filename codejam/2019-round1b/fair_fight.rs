use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let t: usize = sc.read();
    for t in 1..(t + 1) {
        let n = sc.read();
        let k = sc.read::<i64>();
        let c = sc.vec::<i64>(n);
        let d = sc.vec::<i64>(n);
        let ans = n * (n + 1) / 2 - solve(&c, &d, k) - solve(&d, &c, k);
        println!("Case #{}: {}", t, ans);
    }
}

fn solve(c: &[i64], d: &[i64], k: i64) -> usize {
    let c_st = sparse_table::SparseTable::from(c, 0, cmp::max);
    let d_st = sparse_table::SparseTable::from(d, 0, cmp::max);
    let n = c.len();
    (0..n)
        .filter(|&i| c[i] > d[i] + k)
        .map(|i| {
            let mut ng = -1;
            let mut ok = i as i64;
            while ok - ng > 1 {
                let m = ((ok + ng) / 2) as usize;
                if c_st.get(m, i) < c[i] {
                    ok = m as i64;
                } else {
                    ng = m as i64;
                }
            }
            let left = ok;

            let mut ng = left - 1;
            let mut ok = i as i64;
            while ok - ng > 1 {
                let m = (ng + ok) / 2;
                if d_st.get(m as usize, i) < c[i] - k {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let left = ok as usize;

            let mut ng = n + 1;
            let mut ok = i + 1;
            while ng - ok > 1 {
                let m = (ok + ng) / 2;
                if c_st.get(i, m) <= c[i] {
                    ok = m;
                } else {
                    ng = m;
                }
            }

            let mut ng = ok + 1;
            let mut ok = i + 1;
            while ng - ok > 1 {
                let m = (ok + ng) / 2;
                if d_st.get(i, m) < c[i] - k {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let right = ok;

            // [left, right)
            (right - i) * (i - left + 1)
        })
        .sum::<usize>()
}

pub mod sparse_table {
    use std::cmp;

    pub struct SparseTable<T, F> {
        data: Vec<Vec<T>>,
        op: F,
    }

    impl<T, F> SparseTable<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn from(v: &[T], init: T, op: F) -> Self {
            let size = v.len().next_power_of_two();
            let count = size.trailing_zeros() as usize + 1;
            let mut data = vec![vec![init; size]; count];
            for (i, v) in v.iter().cloned().enumerate() {
                data[0][i] = v;
            }
            for c in 1..count {
                for i in 0..size {
                    let next = cmp::min(size - 1, i + (1 << (c - 1)));
                    data[c][i] = op(data[c - 1][i], data[c - 1][next]);
                }
            }

            Self { data: data, op: op }
        }

        /// get the result for [l, r)
        pub fn get(&self, l: usize, r: usize) -> T {
            assert!(l < r);
            let length = r - l;
            if length == 1 {
                return self.data[0][l];
            }
            let block_size = length.next_power_of_two() >> 1;
            let c = block_size.trailing_zeros() as usize;
            let left = self.data[c][l];
            let right = self.data[c][r - block_size];
            (self.op)(left, right)
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
