use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let lr: Vec<(i64, i64)> = (0..n).map(|_| (sc.read(), sc.read())).collect::<Vec<_>>();
    let inv = lr.iter().map(|&(l, r)| (-r, -l)).collect();

    let ans1 = solve(lr);
    let ans2 = solve(inv);
    println!("{}", cmp::max(ans1, ans2));
}

fn solve(lr: Vec<(i64, i64)>) -> i64 {
    let n = lr.len();
    let mut left = vec![];
    let mut right = vec![];
    for (i, &(l, r)) in lr.iter().enumerate() {
        left.push((l, i));
        right.push((r, i));
    }
    left.sort();
    right.sort();

    let mut cur = 0;
    let mut ans = 0;
    let mut left = left.into_iter().rev().peekable();
    let mut right = right.into_iter().peekable();
    let mut used = vec![false; n];
    while left.peek().is_some() || right.peek().is_some() {
        while let Some((_, i)) = left.next() {
            if used[i] {
                continue;
            }
            let (l, r) = lr[i];
            if cur < l {
                ans += l - cur;
                cur = l;
            } else if r < cur {
                ans += cur - r;
                cur = r;
            }
            used[i] = true;
            break;
        }
        while let Some((_, i)) = right.next() {
            if used[i] {
                continue;
            }
            let (l, r) = lr[i];
            if cur < l {
                ans += l - cur;
                cur = l;
            } else if r < cur {
                ans += cur - r;
                cur = r;
            }
            used[i] = true;
            break;
        }
    }
    ans + cur.abs()
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
