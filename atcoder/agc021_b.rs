fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let mut points = vec![];
    for _ in 0..n {
        let x = sc.read();
        let y = sc.read();
        let p = Point { x: x, y: y };
        points.push(p);
    }
    let convex_hull = extract_convex_hull(&points, false);
    let m = convex_hull.len();
    let mut ans = vec![];
    for i in 0..m {
        let left = convex_hull[(m + i - 1) % m];
        let center = convex_hull[i];
        let right = convex_hull[(i + 1) % m];

        let ax = points[left].x - points[center].x;
        let ay = points[left].y - points[center].y;

        let bx = points[right].x - points[center].x;
        let by = points[right].y - points[center].y;

        let mut cos = (ax * bx + ay * by) / (ax * ax + ay * ay).sqrt() / (bx * bx + by * by).sqrt();
        if cos > 1.0 {
            assert!((cos - 1.0).abs() < 1e-6);
            cos = 1.0;
        }
        let t = cos.acos();
        let pi = std::f64::consts::PI;
        ans.push((center, pi - t));
    }

    let sum = ans.iter().map(|&(_, s)| s).sum::<f64>();
    assert!(
        (sum - std::f64::consts::PI * 2.0).abs() < 1e-5,
        "{}",
        (sum - std::f64::consts::PI * 2.0).abs()
    );
    let mut a = vec![0.0; n];
    for (i, s) in ans.into_iter() {
        a[i] = s / sum;
    }

    for ans in a.into_iter() {
        sc.write(format!("{}\n", ans));
    }
}

pub fn extract_convex_hull(points: &Vec<Point>, contain_on_segment: bool) -> Vec<usize> {
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
pub struct Point {
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
    pub fn det(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
