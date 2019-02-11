const INF: i64 = 1e17 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let x: i64 = sc.read();
    let y: i64 = sc.read();
    let n: usize = sc.read();
    let ab = (0..n).map(|_| (sc.read(), sc.read())).collect::<Vec<(i64, i64)>>();

    let per_one = (x + y) / (n as i64);
    let mut v = vec![];
    for x in 0..(per_one + 1) {
        for (i, &(a, b)) in ab.iter().enumerate() {
            let y = per_one - x;
            let happiness = a * x + b * y;
            v.push((happiness, i, x));
        }
    }
    v.sort();

    let mut ng = -1;
    let mut ok = INF;
    while ok - ng > 1 {
        let diff = (ok + ng) / 2;
        if solve(x, n, diff, &v) {
            ok = diff;
        } else {
            ng = diff;
        }
    }

    println!("{}", ok);
}

fn solve(x_sum: i64, n: usize, diff: i64, happiness: &Vec<(i64, usize, i64)>) -> bool {
    let mut low_high: Vec<Option<(_, _)>> = vec![None; n];
    let mut low_sum = 0;
    let mut high_sum = 0;
    let mut bad_count = n;

    let mut tail_iter = happiness.iter().peekable();
    for &(lowest_happiness, i, x) in happiness.iter() {
        while let Some(&&(tail_happiness, tail_index, tail_x)) = tail_iter.peek() {
            if tail_happiness > lowest_happiness + diff { break; }

            match low_high[tail_index] {
                Some((low, high)) => {
                    if low - 1 == tail_x {
                        low_high[tail_index] = Some((low - 1, high));
                        low_sum -= 1;
                    } else if high + 1 == tail_x {
                        low_high[tail_index] = Some((low, high + 1));
                        high_sum += 1;
                    } else {
                        unreachable!();
                    }
                }
                None => {
                    bad_count -= 1;
                    low_high[tail_index] = Some((tail_x, tail_x));
                    low_sum += tail_x;
                    high_sum += tail_x;
                }
            }

            tail_iter.next();
        }

        if bad_count == 0 && low_sum <= x_sum && x_sum <= high_sum {
            return true;
        }


        let (low, high) = low_high[i].unwrap();
        if low == x && high == x {
            bad_count += 1;
            low_high[i] = None;
            low_sum -= low;
            high_sum -= high;
        } else if low == x {
            low_high[i] = Some((low + 1, high));
            low_sum += 1;
        } else if high == x {
            low_high[i] = Some((low, high - 1));
            high_sum -= 1;
        } else {
            unreachable!();
        }
    }
    false
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
