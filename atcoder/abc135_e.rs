fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let k: i64 = sc.read();
    let x: i64 = sc.read();
    let y: i64 = sc.read();

    let (x, y, reversed) = if x.abs() > y.abs() {
        (y, x, true)
    } else {
        (x, y, false)
    };
    let (x, x_neg) = if x < 0 { (-x, true) } else { (x, false) };
    let (y, y_neg) = if y < 0 { (-y, true) } else { (y, false) };

    if k % 2 == 0 && (x + y) % 2 == 1 {
        println!("-1");
        return;
    }

    let ans = solve(k, x, y);

    println!("{}", ans.len());

    for (x, y) in ans.into_iter() {
        let x = if x_neg { -x } else { x };
        let y = if y_neg { -y } else { y };
        let (x, y) = if reversed { (y, x) } else { (x, y) };
        println!("{} {}", x, y);
    }
}

fn solve(k: i64, x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut n = (x + y + k - 1) / k;
    if (k * n - x - y) % 2 == 1 {
        n += 1;
    }

    let ans1 = sub_solve(k, x, y, n);
    let ans2 = sub_solve(k, x, y, n + 1);

    match (ans1, ans2) {
        (Some(ans1), Some(ans2)) => {
            if ans1.len() < ans2.len() {
                ans1
            } else {
                ans2
            }
        }
        (Some(ans), None) | (None, Some(ans)) => ans,
        _ => unreachable!(),
    }
}

fn sub_solve(k: i64, x: i64, y: i64, mut n: i64) -> Option<Vec<(i64, i64)>> {
    let t = (k * n - x - y) / 2;

    let mut s = 0;
    if t + y < k && x + y != k {
        if k % 2 == 0 && t + y + k / 2 >= k {
            n += 1;
            s += k / 2;
        } else {
            n += 2;
            s += k;
        }
    }

    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut ans = vec![];
    if x + y != k {
        while cur_x - k > -t {
            cur_x -= k;
            ans.push((cur_x, cur_y));
        }
        let dx = -t - cur_x;
        cur_y += k - dx.abs();
        cur_x += dx;
        ans.push((cur_x, cur_y));
        while cur_y + k <= y + s {
            cur_y += k;
            ans.push((cur_x, cur_y));
        }

        if s + y > cur_y {
            let dy = s + y - cur_y;
            cur_y += dy;
            cur_x += k - dy;
            ans.push((cur_x, cur_y));
        }
        while cur_x + k <= x {
            cur_x += k;
            ans.push((cur_x, cur_y));
        }
        if cur_x != x || cur_y != y {
            let dx = x - cur_x;
            let dy = k - dx.abs();
            cur_x += dx;
            cur_y -= dy;
            ans.push((cur_x, cur_y));
        }
    } else {
        cur_x = x;
        cur_y = y;
        ans.push((x, y));
    }
    if (cur_x, cur_y) != (x, y) {
        None
    } else {
        validate(&ans, k, x, y);
        Some(ans)
    }
}

fn validate(ans: &Vec<(i64, i64)>, k: i64, x: i64, y: i64) {
    let mut cur = (0, 0);
    for &(x, y) in ans.iter() {
        let (cur_x, cur_y) = cur;
        assert_eq!((cur_x - x).abs() + (cur_y - y).abs(), k);
        cur = (x, y);
    }
    assert_eq!(cur, (x, y));
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
