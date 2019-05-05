use std::cmp;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let ab = (0..n)
        .map(|_| (sc.read::<usize>(), sc.read::<usize>()))
        .collect::<Vec<_>>();
    let left_sum = ab.iter().map(|&(a, _)| a).sum::<usize>();
    let right_sum = ab.iter().map(|&(_, b)| b).sum::<usize>();

    let mut v = vec![];
    for i in 0..n {
        let (a, b) = ab[i];
        v.push((a, i, true));
        v.push((b, i, false));
    }
    v.sort();

    let mut count = vec![0; n];
    let mut sum = 0;
    for i in 0..n {
        let (v, i, _) = v[i];
        sum += v;
        count[i] += 1;
    }

    let mut ans = cmp::min(left_sum, right_sum);
    if count.iter().any(|&x| x == 2) {
        ans = cmp::min(ans, sum);
    } else if v[n - 1].1 != v[n].1 {
        ans = cmp::min(sum - v[n - 1].0 + v[n].0, ans);
    } else {
        if v[n - 2].1 != v[n].1 {
            ans = cmp::min(ans, sum - v[n - 2].0 + v[n].0);
        }
        if v[n - 1].1 != v[n + 1].1 {
            ans = cmp::min(ans, sum - v[n - 1].0 + v[n + 1].0);
        }
    }

    println!("{}", ans);
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
