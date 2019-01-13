fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let mut xy = vec![];
    for _ in 0..n {
        let x: i64 = sc.read();
        let y: i64 = sc.read();
        xy.push((x, y));
    }

    if xy.iter().any(|&(x, y)| (x + y).abs() % 2 == 0)
        && xy.iter().any(|&(x, y)| (x + y).abs() % 2 == 1)
    {
        println!("-1");
        return;
    }

    let mut arms: Vec<i64> = (0..31).rev().map(|i| 1 << i).collect::<Vec<_>>();
    if (xy[0].0 + xy[0].1).abs() % 2 == 0 {
        arms.push(1);
    }

    let arms_sum = arms.iter().sum::<i64>();

    println!("{}", arms.len());
    for (i, &arm) in arms.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arm);
    }
    println!();

    for &(x, y) in xy.iter() {
        let p = x + y;
        let mut cur_sum = arms_sum;
        let mut x_dir = vec![];
        for &arm in arms.iter() {
            if cur_sum - 2 * arm >= p {
                cur_sum -= 2 * arm;
                x_dir.push(-1);
            } else {
                x_dir.push(1);
            }
        }

        let q = x - y;
        let mut cur_sum = arms_sum;
        let mut y_dir = vec![];
        for &arm in arms.iter() {
            if cur_sum - 2 * arm >= q {
                cur_sum -= 2 * arm;
                y_dir.push(-1);
            } else {
                y_dir.push(1);
            }
        }

        let n = x_dir.len();
        for i in 0..n {
            match (x_dir[i], y_dir[i]) {
                (1, 1) => print!("R"),
                (1, -1) => print!("U"),
                (-1, 1) => print!("D"),
                (-1, -1) => print!("L"),
                _ => unreachable!(),
            }
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
