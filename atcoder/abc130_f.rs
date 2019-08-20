fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    let mut left = State::new();
    let mut right = State::new();
    let mut top = State::new();
    let mut bottom = State::new();

    for _ in 0..n {
        let x: f64 = sc.read();
        let y: f64 = sc.read();
        let c = sc.chars()[0];
        match c {
            'R' => {
                right.put_increasing(x);
                left.put_decreasing(-x);
                top.put_static(y);
                bottom.put_static(-y);
            }
            'L' => {
                right.put_decreasing(x);
                left.put_increasing(-x);
                top.put_static(y);
                bottom.put_static(-y);
            }
            'D' => {
                top.put_decreasing(y);
                bottom.put_increasing(-y);
                right.put_static(x);
                left.put_static(-x);
            }
            'U' => {
                top.put_increasing(y);
                bottom.put_decreasing(-y);
                right.put_static(x);
                left.put_static(-x);
            }
            _ => unreachable!(),
        }
    }

    let mut events = top.get_events();
    events.extend(bottom.get_events());
    events.extend(left.get_events());
    events.extend(right.get_events());
    events.push(0.0);

    let mut ans = std::f64::MAX;
    for t in events.into_iter() {
        if t < 0.0 {
            continue;
        }
        let right = right.get_value(t);
        let left = left.get_value(t);
        let top = top.get_value(t);
        let bottom = bottom.get_value(t);
        let s = (left + right).abs() * (top + bottom).abs();
        ans = ans.min(s);
    }
    println!("{}", ans);
}

struct State {
    decreasing: f64,
    increasing: f64,
    static_point: f64,
}

impl State {
    fn new() -> State {
        State {
            decreasing: std::f64::MIN,
            increasing: std::f64::MIN,
            static_point: std::f64::MIN,
        }
    }

    fn put_static(&mut self, p: f64) {
        self.static_point = self.static_point.max(p);
    }
    fn put_increasing(&mut self, p: f64) {
        self.increasing = self.increasing.max(p);
    }
    fn put_decreasing(&mut self, p: f64) {
        self.decreasing = self.decreasing.max(p);
    }

    fn get_events(&self) -> Vec<f64> {
        let mut result = vec![];
        if self.decreasing != std::f64::MIN && self.static_point != std::f64::MIN {
            result.push(self.decreasing - self.static_point);
        }
        if self.static_point != std::f64::MIN && self.increasing != std::f64::MIN {
            result.push(self.static_point - self.increasing);
        }
        if self.decreasing != std::f64::MIN && self.increasing != std::f64::MIN {
            result.push((self.decreasing - self.increasing) / 2.0);
        }
        result
    }

    fn get_value(&self, t: f64) -> f64 {
        let v1 = self.decreasing - t;
        let v2 = self.increasing + t;
        let v3 = self.static_point;
        v1.max(v2).max(v3)
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
