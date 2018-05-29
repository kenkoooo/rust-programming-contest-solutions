fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let points = (0..n)
        .map(|_| Point {
            x: sc.read(),
            y: sc.read(),
        })
        .collect();
    let convex_hull = extract_convex_hull(&points, true);

    let mut ans = vec![0.0; points.len()];

    let n = convex_hull.len();
    for i in 0..n {
        let prev = points[convex_hull[(i + n - 1) % n]];
        let cur = points[convex_hull[i]];
        let next = points[convex_hull[(i + 1) % n]];

        let a = next - cur;
        let b = prev - cur;

        let cos = a.product(&b) / a.abs() / b.abs();
        let rad = cos.acos();
        ans[convex_hull[i]] = (1.0 - rad / std::f64::consts::PI) / 2.0;
    }

    for &a in &ans {
        println!("{}", a);
    }
}

fn extract_convex_hull(points: &Vec<Point>, contain_on_segment: bool) -> Vec<usize> {
    let n = points.len();
    if n <= 1 {
        return vec![0];
    }

    let mut ps: Vec<usize> = (0..n).collect();
    ps.sort_by(|&a, &b| {
        if points[a].x == points[b].x {
            points[a].y.partial_cmp(&points[b].y).unwrap()
        } else {
            points[a].x.partial_cmp(&points[b].x).unwrap()
        }
    });

    let mut qs: Vec<usize> = Vec::new();
    for &i in &ps {
        while qs.len() > 1 {
            let k = qs.len();
            let det = (points[qs[k - 1]] - points[qs[k - 2]]).det(&(points[i] - points[qs[k - 1]]));
            if det < 0.0 || (det <= 0.0 && !contain_on_segment) {
                qs.pop();
            } else {
                break;
            }
        }
        qs.push(i);
    }

    let t = qs.len();
    for i in (0..(n - 1)).rev() {
        let i = ps[i];
        while qs.len() > t {
            let k = qs.len();
            let det = (points[qs[k - 1]] - points[qs[k - 2]]).det(&(points[i] - points[qs[k - 1]]));
            if det < 0.0 || (det <= 0.0 && !contain_on_segment) {
                qs.pop();
            } else {
                break;
            }
        }
        qs.push(i);
    }

    qs.pop();
    return qs;
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn det(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn product(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
