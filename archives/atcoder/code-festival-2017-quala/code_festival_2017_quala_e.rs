use std::cmp;

fn solve(x: i64, n: usize, v: &Vec<(i64, usize, i64)>, d: i64) -> bool {
    let mut status = vec![(-1, -1); n];

    let mut bad = n;
    let mut lower_sum = 0;
    let mut upper_sum = 0;

    let mut r = 0;
    for &(left_value, left_id, left_num) in v.iter() {
        while r < v.len() && v[r].0 <= left_value + d {
            let (_, id, num) = v[r];
            let (lower, upper) = status[id];
            if lower == -1 {
                status[id] = (num, num);
                bad -= 1;
                lower_sum += num;
                upper_sum += num;
            } else if upper == num - 1 {
                status[id] = (lower, num);
                upper_sum += 1;
            } else {
                assert_eq!(lower, num + 1);
                status[id] = (num, upper);
                lower_sum -= 1;
            }

            r += 1;
        }
        if bad == 0 && lower_sum <= x && x <= upper_sum {
            return true;
        }

        let (lower, upper) = status[left_id];
        if lower == left_num && upper == left_num {
            status[left_id] = (-1, -1);
            lower_sum -= left_num;
            upper_sum -= left_num;
            bad += 1;
        } else if lower == left_num {
            status[left_id] = (lower + 1, upper);
            lower_sum += 1;
        } else {
            status[left_id] = (lower, upper - 1);
            upper_sum -= 1;
        }
    }
    false
}

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let x: i64 = sc.read();
    let y: i64 = sc.read();
    let n: usize = sc.read();
    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = sc.read();
        b[i] = sc.read();
    }

    let s = (x + y) / (n as i64);

    let mut v = vec![];
    for i in 0..n {
        let a = a[i];
        let b = b[i];
        for x in 0..(s + 1) {
            v.push((a * x + b * (s - x), i, x));
        }
    }
    v.sort();

    let mut ok = 1e15 as i64;
    let mut ng = -1;
    while ok - ng > 1 {
        let d = (ok + ng) / 2;
        if solve(x, n, &v, d) {
            ok = d;
        } else {
            ng = d;
        }
    }

    println!("{}", ok);
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
