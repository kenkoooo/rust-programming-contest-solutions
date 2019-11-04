use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut seg = vec![];
    for _ in 0..n {
        let l = sc.read::<i64>();
        let r = sc.read::<i64>() + 1;
        seg.push((l, r));
    }

    let max_l = seg.iter().map(|&(l, _)| l).max().unwrap();
    let min_r = seg.iter().map(|&(_, r)| r).min().unwrap();
    let max_seg = seg.iter().map(|&(l, r)| r - l).max().unwrap();
    let ans1 = max_seg + cmp::max(min_r - max_l, 0);

    let mut seg = seg
        .into_iter()
        .map(|(l, r)| {
            (
                cmp::max(r - max_l, 0),
                -cmp::max(min_r - l, 0),
                cmp::max(min_r - l, 0),
            )
        })
        .collect::<Vec<_>>();
    seg.sort();

    let (_, _, mut min_b) = seg[0];
    let (mut min_a, _, _) = seg[1];
    let mut ans2 = min_a + min_b;
    for i in 1..(n - 1) {
        let (_, _, b) = seg[i];
        min_b = cmp::min(min_b, b);
        let (a, _, _) = seg[i + 1];
        min_a = a;
        ans2 = cmp::max(ans2, min_a + min_b);
    }

    println!("{}", cmp::max(ans1, ans2));
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
