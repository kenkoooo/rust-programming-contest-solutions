const INF: f64 = 1e18;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let mut const_x = Range::new();
    let mut const_y = Range::new();

    let mut up_x = Range::new();
    let mut down_x = Range::new();

    let mut up_y = Range::new();
    let mut down_y = Range::new();

    let n: usize = sc.read();
    for _ in 0..n {
        let x: f64 = sc.read();
        let y: f64 = sc.read();
        let d = sc.chars()[0];
        match d {
            'R' => {
                up_x.set(x);
            }
            'L' => {
                down_x.set(x);
            }
            'U' => {
                const_x.set(x);
            }
            'D' => {
                const_x.set(x);
            }
            _ => unreachable!(),
        }
        match d {
            'R' => {
                const_y.set(y);
            }
            'L' => {
                const_y.set(y);
            }
            'U' => {
                up_y.set(y);
            }
            'D' => {
                down_y.set(y);
            }
            _ => unreachable!(),
        }
    }

    let mut times = vec![0.0];
    add_time3(&const_x, &up_x, &down_x, &mut times);
    add_time3(&const_y, &up_y, &down_y, &mut times);

    let candidates = times
        .into_iter()
        .map(|time| {
            let dx = length(&const_x, &up_x, &down_x, time);
            let dy = length(&const_y, &up_y, &down_y, time);
            assert!(dx >= 0.0);
            assert!(dy >= 0.0);
            dx * dy
        })
        .collect::<Vec<_>>();

    let mut ans = INF;
    for c in candidates.into_iter() {
        if ans > c {
            ans = c;
        }
    }
    assert!(ans >= 0.0);

    println!("{}", ans);
}

fn add_time3(const_y: &Range, up_y: &Range, down_y: &Range, times: &mut Vec<f64>) {
    add_time2(up_y, const_y, 1.0, times);
    add_time2(up_y, down_y, 2.0, times);
    add_time2(const_y, down_y, 1.0, times);
}

fn add_time2(up: &Range, down: &Range, step: f64, times: &mut Vec<f64>) {
    add_time(up.max, down.max, step, times);
    add_time(up.max, down.min, step, times);
    add_time(up.min, down.max, step, times);
    add_time(up.min, down.min, step, times);
}

fn add_time(up: f64, down: f64, step: f64, times: &mut Vec<f64>) {
    if up.is_finite() && down.is_finite() && up < down {
        times.push((down - up) / step);
    }
}

fn length(const_y: &Range, up_y: &Range, down_y: &Range, time: f64) -> f64 {
    const_y.add(up_y.at(time)).add(down_y.at(-time)).dist()
}

struct Range {
    max: f64,
    min: f64,
}

impl Range {
    fn new() -> Self {
        Range {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }

    fn set(&mut self, v: f64) {
        self.min = self.min.min(v);
        self.max = self.max.max(v);
    }

    fn at(&self, time: f64) -> Self {
        Range {
            min: self.min + time,
            max: self.max + time,
        }
    }

    fn add(&self, other: Range) -> Self {
        Range {
            max: self.max.max(other.max),
            min: self.min.min(other.min),
        }
    }

    fn dist(&self) -> f64 {
        assert!(self.max.is_finite());
        assert!(self.min.is_finite());
        self.max - self.min
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
