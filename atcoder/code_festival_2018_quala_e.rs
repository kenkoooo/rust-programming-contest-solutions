const INF: i64 = 1e17 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let x: i64 = sc.read();
    let y: i64 = sc.read();
    let n: usize = sc.read();
    let ab: Vec<(i64, i64)> = (0..n).map(|_| (sc.read(), sc.read())).collect();
    let n = ab.len() as i64;
    let per_one = (x + y) / n;
    let mut candidates = vec![];
    for i in 0..n {
        let (a, b) = ab[i as usize];
        for j in 0..(per_one + 1) {
            let happiness = a * j + b * (per_one - j);
            candidates.push((happiness, j, i as usize));
        }
    }
    candidates.sort();

    let mut ng = -1;
    let mut ok = INF;
    while ok - ng > 1 {
        let difference = (ok + ng) / 2;
        if solve(difference, x, n, &candidates) {
            ok = difference;
        } else {
            ng = difference;
        }
    }
    println!("{}", ok);
}

fn solve(difference: i64, x: i64, n: i64, candidates: &Vec<(i64, i64, usize)>) -> bool {
    let mut from_to = vec![(-1, -1); n as usize];
    let mut alive = 0;
    let mut from_sum = 0;
    let mut to_sum = 0;

    let mut tail = candidates.iter().peekable();
    for &(head_happiness, head_j, head_i) in candidates.iter() {
        while let Some(&&(tail_hapiness, tail_j, tail_i)) = tail.peek() {
            if tail_hapiness > head_happiness + difference {
                break;
            }

            if from_to[tail_i].0 == -1 {
                from_to[tail_i] = (tail_j, tail_j);
                alive += 1;
                from_sum += tail_j;
                to_sum += tail_j;
            } else if from_to[tail_i].0 - 1 == tail_j {
                from_to[tail_i].0 -= 1;
                from_sum -= 1;
            } else if from_to[tail_i].1 + 1 == tail_j {
                from_to[tail_i].1 += 1;
                to_sum += 1;
            } else {
                unreachable!();
            }
            tail.next();
        }

        if alive == n && from_sum <= x && x <= to_sum {
            return true;
        }

        if from_to[head_i].0 == from_to[head_i].1 {
            assert_eq!(from_to[head_i], (head_j, head_j));
            from_to[head_i] = (-1, -1);
            alive -= 1;
            from_sum -= head_j;
            to_sum -= head_j;
        } else if from_to[head_i].0 == head_j {
            from_to[head_i].0 += 1;
            from_sum += 1;
        } else if from_to[head_i].1 == head_j {
            from_to[head_i].1 -= 1;
            to_sum -= 1;
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
