fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let x: i64 = sc.read();
    let y: i64 = sc.read();
    let n: usize = sc.read();
    let ab = (0..n).map(|_| (sc.read(), sc.read())).collect::<Vec<(i64, i64)>>();

    let per_one = (x + y) / (n as i64);
    let mut v = vec![];
    for (i, &(a, b)) in ab.iter().enumerate() {
        for x in 0..(per_one + 1) {
            let y = per_one - x;
            let happiness = a * x + b * y;
            v.push((happiness, i, x));
        }
    }
    v.sort();

    let mut ok = 1e15 as i64;
    let mut ng = -1;
    while ok - ng > 1 {
        let diff = (ok + ng) / 2;
        if calc(x, n, &v, diff) {
            ok = diff;
        } else {
            ng = diff;
        }
    }

    println!("{}", ok);
}

fn calc(x_sum: i64, n: usize, v: &Vec<(i64, usize, i64)>, diff: i64) -> bool {
    let mut status = vec![(-1, -1); n];

    let mut lower_sum = 0;
    let mut upper_sum = 0;
    let mut bad_count = n;
    let mut right_head = 0;
    for &(lowest_happiness, left_id, left_x) in v.iter() {
        while right_head < v.len() {
            let (max_happiness, head_id, new_x) = v[right_head];
            if max_happiness > lowest_happiness + diff { break; }

            let (lower, upper) = status[head_id];
            if lower == -1 && upper == -1 {
                status[head_id] = (new_x, new_x);
                lower_sum += new_x;
                upper_sum += new_x;
                bad_count -= 1;
            } else if lower - 1 == new_x {
                status[head_id] = (lower - 1, upper);
                lower_sum -= 1;
            } else if upper + 1 == new_x {
                status[head_id] = (lower, upper + 1);
                upper_sum += 1;
            } else {
                unreachable!();
            }

            right_head += 1;
        }

        if bad_count == 0 && lower_sum <= x_sum && x_sum <= upper_sum {
            return true;
        }

        let (lower, upper) = status[left_id];
        if lower == left_x && upper == left_x {
            bad_count += 1;
            lower_sum -= left_x;
            upper_sum -= left_x;
            status[left_id] = (-1, -1);
        } else if lower == left_x {
            status[left_id] = (lower + 1, upper);
            lower_sum += 1;
        } else if upper == left_x {
            status[left_id] = (lower, upper - 1);
            upper_sum -= 1;
        } else {
            unreachable!();
        }
    }

    false
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
