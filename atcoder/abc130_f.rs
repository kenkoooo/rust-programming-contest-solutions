fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    let mut right_side = Side::new();
    let mut left_side = Side::new();
    let mut head_side = Side::new();
    let mut bottom_side = Side::new();

    for _ in 0..n {
        let x = sc.read::<f64>();
        let y = sc.read::<f64>();
        let d = sc.chars()[0];
        match d {
            'R' => {
                right_side.increasing = right_side.increasing.max(x);
                left_side.decreasing = left_side.decreasing.max(-x);
                head_side.solid = head_side.solid.max(y);
                bottom_side.solid = bottom_side.solid.max(-y);
            }
            'L' => {
                right_side.decreasing = right_side.decreasing.max(x);
                left_side.increasing = left_side.increasing.max(-x);
                head_side.solid = head_side.solid.max(y);
                bottom_side.solid = bottom_side.solid.max(-y);
            }
            'U' => {
                head_side.increasing = head_side.increasing.max(y);
                bottom_side.decreasing = bottom_side.decreasing.max(-y);
                right_side.solid = right_side.solid.max(x);
                left_side.solid = left_side.solid.max(-x);
            }
            'D' => {
                head_side.decreasing = head_side.decreasing.max(y);
                bottom_side.increasing = bottom_side.increasing.max(-y);
                right_side.solid = right_side.solid.max(x);
                left_side.solid = left_side.solid.max(-x);
            }
            _ => unreachable!(),
        }
    }

    let mut events = vec![0.0];
    head_side.add(&mut events);
    left_side.add(&mut events);
    right_side.add(&mut events);
    bottom_side.add(&mut events);

    let mut ans = std::f64::MAX;
    for t in events.into_iter() {
        let x_max = right_side.get(t);
        let x_min = -left_side.get(t);
        let y_max = head_side.get(t);
        let y_min = -bottom_side.get(t);
        let s = (x_max - x_min) * (y_max - y_min);
        ans = ans.min(s);
    }
    println!("{}", ans);
}

struct Side {
    increasing: f64,
    decreasing: f64,
    solid: f64,
}

impl Side {
    fn new() -> Side {
        Side {
            increasing: std::f64::MIN,
            decreasing: std::f64::MIN,
            solid: std::f64::MIN,
        }
    }

    fn add(&self, events: &mut Vec<f64>) {
        if self.increasing != std::f64::MIN && self.solid != std::f64::MIN {
            let d = self.solid - self.increasing;
            if d > 0.0 {
                events.push(d);
            }
        }
        if self.decreasing != std::f64::MIN && self.solid != std::f64::MIN {
            let d = self.decreasing - self.solid;
            if d > 0.0 {
                events.push(d);
            }
        }
        if self.increasing != std::f64::MIN && self.decreasing != std::f64::MIN {
            let d = (self.decreasing - self.increasing) / 2.0;
            if d > 0.0 {
                events.push(d);
            }
        }
    }

    fn get(&self, t: f64) -> f64 {
        let x1 = self.increasing + t;
        let x2 = self.decreasing - t;
        self.solid.max(x1).max(x2)
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
