use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let p: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.chars()
                .into_iter()
                .map(|c| c as usize - 'a' as usize)
                .collect()
        })
        .collect();

    let prefix_suffix: Vec<_> = p
        .iter()
        .map(|p| {
            let length = p.len();
            let head = p[0];
            let tail = p[length - 1];
            let prefix = p.iter().take_while(|&&c| c == head).count();
            let suffix = p.iter().rev().take_while(|&&c| c == tail).count();
            (head, tail, prefix, suffix)
        })
        .collect();

    let t = (0..n)
        .rev()
        .skip_while(|&i| {
            let (_, _, prefix, _) = prefix_suffix[i];
            prefix == p[i].len()
        })
        .next()
        .unwrap_or(0);

    let mut ans = vec![[0; 26]; n];
    for i in t..n {
        let (head, tail, prefix, suffix) = prefix_suffix[i];
        let length = p[i].len();

        if i == 0 {
            ans[i] = calc_longest(&p[i]);
        } else if i == t {
            assert!(prefix != length);
            let mut has = [false; 26];
            let mut max = calc_longest(&p[i]);
            for p in p[0..i].iter() {
                for &c in p.iter() {
                    has[c] = true;
                }
            }

            if head == tail && has[head] {
                update_max(&mut max[head], suffix + 1 + prefix);
            }
            if head != tail {
                if has[head] {
                    max[head] += 1;
                }
                if has[tail] {
                    max[tail] += 1;
                }
            }
            ans[i] = max;
        } else {
            let inner = ans[i - 1];
            let mut max = calc_longest(&p[i]);
            update_max(
                &mut max[head],
                cmp::min((inner[head] + 1) * length + inner[head], 1e9 as usize),
            );
            for i in 0..26 {
                if inner[i] > 0 {
                    update_max(&mut max[i], 1);
                }
            }
            ans[i] = max;
        }
    }

    println!("{}", ans[n - 1].iter().max().unwrap());
}

fn calc_longest(last: &Vec<usize>) -> [usize; 26] {
    let mut cur = 1;
    let n = last.len();
    let mut max = [0; 26];
    for i in 1..n {
        if last[i - cur] == last[i] {
            cur += 1;
        } else {
            update_max(&mut max[last[i - cur]], cur);
            cur = 1;
        }
    }
    update_max(&mut max[last[last.len() - 1]], cur);
    max
}

fn update_max<T: PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b
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
            .map(|b| match b {
                Ok(b) => b,
                _ => panic!("{:?}", b),
            })
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        match unsafe { std::str::from_utf8_unchecked(&buf) }.parse() {
            Ok(r) => r,
            _ => panic!("Parse Error: {:?}", String::from_utf8(buf)),
        }
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
