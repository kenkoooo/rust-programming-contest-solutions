use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let lr = (0..n)
        .map(|_| (sc.read(), sc.read()))
        .collect::<Vec<(i64, i64)>>();

    let mut left = lr
        .iter()
        .enumerate()
        .map(|(i, &(l, _))| (l, i))
        .collect::<Vec<(i64, usize)>>();
    left.sort();
    left.reverse();
    let left = left.iter().map(|&(_, i)| i).collect::<Vec<_>>();
    let mut right = lr
        .iter()
        .enumerate()
        .map(|(i, &(_, r))| (r, i))
        .collect::<Vec<(i64, usize)>>();
    right.sort();
    let right = right.iter().map(|&(_, i)| i).collect::<Vec<_>>();

    let ans1 = solve(&left, &right, &lr);
    let ans2 = solve(&right, &left, &lr);
    println!("{}", cmp::max(ans1, ans2));
}

fn solve(left: &Vec<usize>, right: &Vec<usize>, lr: &Vec<(i64, i64)>) -> i64 {
    let n = lr.len();
    let mut ans = 0;
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut used = vec![false; n];
    let mut cur = 0;
    for i in 0..n {
        let iter = if i % 2 == 0 {
            &mut left_iter
        } else {
            &mut right_iter
        };

        while let Some(&index) = iter.next() {
            if used[index] {
                continue;
            }
            used[index] = true;
            let (left, right) = lr[index];
            if cur < left {
                ans += left - cur;
                cur = left;
            } else if right < cur {
                ans += cur - right;
                cur = right;
            }
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
