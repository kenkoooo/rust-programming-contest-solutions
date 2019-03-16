fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();
    if (a.count_ones() & 1) == (b.count_ones() & 1) {
        println!("NO");
        return;
    }
    let ans = solve(n, a, b);
    println!("YES");
    for (i, ans) in ans.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans);
    }
    println!();
}

fn solve(n: usize, a: usize, b: usize) -> Vec<usize> {
    if n == 1 {
        vec![a, b]
    } else {
        assert!((a.count_ones() & 1) != (b.count_ones() & 1));
        let x = (0..)
            .filter(|&i| (a & (1 << i)) != (b & (1 << i)))
            .next()
            .unwrap();
        let na = ((a >> (x + 1)) << x) + (((1 << x) - 1) & a);
        let nb = ((b >> (x + 1)) << x) + (((1 << x) - 1) & b);
        assert_eq!((na.count_ones() & 1), (nb.count_ones() & 1));

        let c = na ^ 1;
        let q = solve(n - 1, na, c);
        let r = solve(n - 1, c, nb);
        let m = 1 << n;
        let mut ans = vec![0; m];
        for i in 0..(m / 2) {
            let q = q[i];
            ans[i] = (a & (1 << x)) + ((q >> x) << (x + 1)) + (((1 << x) - 1) & q);
        }
        for i in (m / 2)..m {
            let r = r[i - (m / 2)];
            ans[i] = (b & (1 << x)) + ((r >> x) << (x + 1)) + (((1 << x) - 1) & r);
        }
        ans
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
