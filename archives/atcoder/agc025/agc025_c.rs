use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let mut right = vec![];
    let mut left = vec![];
    for _ in 0..n {
        left.push(sc.read::<i64>());
        right.push(sc.read::<i64>());
    }

    let left_inv = left.iter().map(|&left| -left).collect();
    let right_inv = right.iter().map(|&right| -right).collect();
    let ans1 = solve(left, right);
    let ans2 = solve(right_inv, left_inv);
    println!("{}", cmp::max(ans1, ans2));
}
fn solve(left: Vec<i64>, right: Vec<i64>) -> i64 {
    let n = left.len();

    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();
    for i in 0..n {
        left_heap.push((left[i], i));
        right_heap.push((-right[i], i));
    }

    let mut ans = 0;
    let mut cur = 0;
    let mut used = vec![false; n];
    for i in 0..n {
        let k = if i % 2 == 0 {
            {
                let mut result;
                loop {
                    let (_, k) = left_heap.pop().unwrap();
                    if used[k] {
                        continue;
                    }
                    used[k] = true;
                    result = k;
                    break;
                }
                result
            }
        } else {
            {
                let mut result;
                loop {
                    let (_, k) = right_heap.pop().unwrap();
                    if used[k] {
                        continue;
                    }
                    used[k] = true;
                    result = k;
                    break;
                }
                result
            }
        };

        if left[k] <= cur && cur <= right[k] {
            continue;
        }

        if (left[k] - cur).abs() < (right[k] - cur).abs() {
            ans += (left[k] - cur).abs();
            cur = left[k];
        } else {
            ans += (right[k] - cur).abs();
            cur = right[k];
        }
    }
    ans += cur.abs();
    ans
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
