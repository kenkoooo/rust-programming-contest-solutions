fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let mut a = sc
        .vec::<i64>(n)
        .into_iter()
        .enumerate()
        .map(|(i, a)| (-a, i))
        .collect::<Vec<_>>();
    a.sort();
    let mut ans = vec![0; n];

    let mut garbage_count = 0;
    let mut garbage_num = -a[0].0;
    let mut stack = vec![];
    let mut cur = a[0].1;
    for &(a, i) in a.iter() {
        let a = -a;
        if cur > i {
            ans[cur] += garbage_count * (garbage_num - a);
            garbage_num = a;
            while let Some(b) = stack.pop() {
                ans[cur] += b - a;
                garbage_count += 1;
            }
            cur = i;
        }
        stack.push(a);
    }
    ans[cur] += stack.into_iter().sum::<i64>();
    ans[cur] += garbage_count * garbage_num;
    for c in ans.into_iter() {
        println!("{}", c);
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
