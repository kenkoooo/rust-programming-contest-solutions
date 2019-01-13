use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n = sc.read();
    let ab = (0..n)
        .map(|_| (sc.read(), sc.read()))
        .collect::<Vec<(i64, i64)>>();

    let a_sum = ab.iter().map(|&(a, _)| a).sum::<i64>();
    let b_sum = ab.iter().map(|&(_, b)| b).sum::<i64>();

    let mut v = vec![];
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((a, i));
        v.push((b, i));
    }
    v.sort();

    let mut used_count = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        let (t, i) = v[i];
        used_count[i] += 1;
        ans += t;
    }

    if used_count.iter().any(|&c| c == 2) {
        println!("{}", cmp::min(cmp::min(a_sum, b_sum), ans));
    } else {
        let mut min = 1e15 as i64;
        for i in 0..n {
            let (t, i) = v[i];
            let ans = ans - t;
            if i == v[n].1 {
                min = cmp::min(min, ans + v[n + 1].0);
            } else {
                min = cmp::min(min, ans + v[n].0);
            }
        }
        println!("{}", cmp::min(cmp::min(a_sum, b_sum), min));
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
