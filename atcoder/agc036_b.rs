use std::usize::MAX;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let k: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let max_a = *a.iter().max().unwrap();
    let mut add = vec![0; n];
    let mut last: Vec<usize> = vec![MAX; max_a + 1];
    for i in 0..(2 * n) {
        let a = a[i % n];
        if last[a] != MAX {
            if last[a] < n {
                add[last[a]] = i - last[a];
            }
        }
        last[a] = i;
    }
    assert!(add.iter().all(|&add| add > 0));

    let (first, second) = find_cycle(&add);
    let length = (n * k - first) % (second - first) + first;
    let mut start = 0;
    loop {
        let next = add[start % n] + 1 + start;
        if next >= length {
            break;
        }
        start = next;
    }

    let mut set: Vec<bool> = vec![false; max_a + 1];
    let mut ans: Vec<usize> = vec![];
    for i in start..length {
        let a = a[i % n];
        if set[a] {
            while let Some(tail) = ans.pop() {
                assert!(set[tail]);
                set[tail] = false;
                if tail == a {
                    break;
                }
            }
        } else {
            set[a] = true;
            ans.push(a);
        }
    }
    for (i, ans) in ans.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans);
    }
    println!();
}

fn find_cycle(add: &Vec<usize>) -> (usize, usize) {
    let n = add.len();
    let mut last: Vec<usize> = vec![MAX; n];
    let mut cur = 0;
    loop {
        if last[cur % n] != MAX {
            return (last[cur % n], cur);
        }

        last[cur % n] = cur;
        cur += add[cur % n] + 1;
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
