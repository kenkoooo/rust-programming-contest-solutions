fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let x: u64 = sc.read();
    let y: u64 = sc.read();
    let n: u64 = sc.read();
    let one = (x + y) / n;

    let mut candidates = vec![];
    for i in 0..(n as usize) {
        let a: u64 = sc.read();
        let b: u64 = sc.read();
        for x in 0..(one + 1) {
            let y = one - x;
            let happiness = x * a + b * y;
            candidates.push((happiness, i, x));
        }
    }
    candidates.sort();

    if solve(x, one, 0, &candidates) {
        println!("0");
        return;
    }

    let mut ok = 1e15 as u64;
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        if solve(x, one, m, &candidates) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}

fn solve(total_x: u64, one: u64, difference: u64, candidates: &Vec<(u64, usize, u64)>) -> bool {
    let n = candidates.len() / (one as usize + 1);
    let mut ranges: Vec<Option<(u64, u64)>> = vec![None; n];
    let mut lower_sum = 0;
    let mut upper_sum = 0;
    let mut count = 0;

    let mut tail = 0;
    for &(lowest, i, x) in candidates.iter() {
        while tail < candidates.len() && candidates[tail].0 <= difference + lowest {
            let (_, i, x) = candidates[tail];
            match ranges[i] {
                Some((lower, upper)) => {
                    if x + 1 == lower {
                        ranges[i] = Some((x, upper));
                        lower_sum -= 1;
                    } else if x == upper + 1 {
                        ranges[i] = Some((lower, x));
                        upper_sum += 1;
                    } else {
                        unreachable!();
                    }
                }
                None => {
                    ranges[i] = Some((x, x));
                    lower_sum += x;
                    upper_sum += x;
                    count += 1;
                }
            }
            tail += 1;
        }

        if count == n && lower_sum <= total_x && total_x <= upper_sum {
            return true;
        }

        let (lower, upper) = ranges[i].unwrap();
        if lower == upper {
            assert_eq!(lower, x);
            ranges[i] = None;
            lower_sum -= x;
            upper_sum -= x;
            count -= 1;
        } else if lower == x {
            ranges[i] = Some((lower + 1, upper));
            lower_sum += 1;
        } else if upper == x {
            ranges[i] = Some((lower, upper - 1));
            upper_sum -= 1;
        } else {
            unreachable!()
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
