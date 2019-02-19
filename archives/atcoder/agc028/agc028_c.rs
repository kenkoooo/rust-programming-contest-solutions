use std::cmp;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let ab = (0..n)
        .map(|_| (sc.read(), sc.read()))
        .collect::<Vec<(u64, u64)>>();
    let a_sum = ab.iter().map(|&(a, _)| a).sum::<u64>();
    let b_sum = ab.iter().map(|&(_, b)| b).sum::<u64>();

    println!("{}", cmp::min(cmp::min(a_sum, b_sum), solve(ab)));
}

fn solve(ab: Vec<(u64, u64)>) -> u64 {
    let n = ab.len();

    let mut v = vec![];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        v.push((a, true, i));
        v.push((b, false, i));
    }
    v.sort();

    let mut count = vec![0; n];
    let mut sum = 0;
    for &(x, _, i) in v.iter().take(n) {
        count[i] += 1;
        sum += x;
    }
    if count.into_iter().any(|c| c == 2) {
        sum
    } else {
        let first = v
            .iter()
            .rev()
            .skip(n)
            .filter(|&&(_, _, i)| i != v[n].2)
            .map(|&(x, _, _)| x)
            .next()
            .unwrap();
        let second = v
            .iter()
            .rev()
            .skip(n)
            .filter(|&&(_, _, i)| i != v[n + 1].2)
            .map(|&(x, _, _)| x)
            .next()
            .unwrap();
        cmp::min(sum - first + v[n].0, sum - second + v[n + 1].0)
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
