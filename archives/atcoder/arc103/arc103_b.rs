fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let mut x: Vec<i64> = vec![];
    let mut y: Vec<i64> = vec![];
    for _ in 0..n {
        x.push(sc.read());
        y.push(sc.read());
    }

    for i in 1..n {
        if (x[0] + y[0]).abs() % 2 != (x[i] + y[i]).abs() % 2 {
            println!("-1");
            return;
        }
    }

    let mut arms = vec![];
    if (x[0] + y[0]).abs() % 2 == 0 {
        arms.push(1);
    }
    for i in 0..31 {
        arms.push(1 << i);
    }
    let m = arms.len();
    println!("{}", m);
    for i in 0..m {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arms[i]);
    }
    println!();

    let sum: i64 = arms.iter().map(|&a| a).sum();
    for (u, v) in (0..n).map(|i| (x[i] + y[i], x[i] - y[i])) {
        let calc_up = |u: i64| {
            assert!((u + sum) % 2 == 0);
            let mut u = (u + sum) / 2;
            let mut p = vec![false; m];
            for i in (0..m).rev() {
                if u >= arms[i] {
                    u -= arms[i];
                    p[i] = true;
                }
            }
            p
        };
        let u_up = calc_up(u);
        let v_up = calc_up(v);
        for i in 0..m {
            let dir = match (u_up[i], v_up[i]) {
                (true, true) => 'R',
                (false, false) => 'L',
                (false, true) => 'D',
                (true, false) => 'U',
            };
            print!("{}", dir);
        }
        println!();
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
