use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let max_a = a.iter().cloned().max().unwrap();
    let mut next = vec![vec![]; max_a + 1];
    let mut pos = vec![0; n];
    for (i, &a) in a.iter().enumerate() {
        pos[i] = next[a].len();
        next[a].push(i);
    }

    let mut jumped = false;
    let mut cur_t = 0;
    let mut cur_pos = 0;
    let mut prev = vec![std::usize::MAX; n];
    prev[0] = cur_t;
    loop {
        let pos = pos[cur_pos];
        if next[a[cur_pos]].len() == pos + 1 {
            if cur_t == k - 1 {
                let mut ans = vec![];
                let mut set = BTreeSet::new();
                for i in cur_pos..n {
                    if set.contains(&a[i]) {
                        while let Some(tail) = ans.pop() {
                            set.remove(&tail);
                            if tail == a[i] {
                                break;
                            }
                        }
                    } else {
                        ans.push(a[i]);
                        set.insert(a[i]);
                    }
                }

                for (i, a) in ans.into_iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", a);
                }
                println!();
                return;
            }
            cur_t += 1;
            cur_pos = next[a[cur_pos]][0];
        } else {
            cur_pos = next[a[cur_pos]][pos + 1]
        }
        cur_pos += 1;
        if cur_pos == n {
            if cur_t == k - 1 {
                println!();
                return;
            }
            cur_t += 1;
            cur_pos = 0;
        }
        if !jumped && prev[cur_pos] < cur_t {
            let dt = cur_t - prev[cur_pos];
            let z = (k - prev[cur_pos] - 1) / dt;
            cur_t = prev[cur_pos] + dt * z;
            assert!(cur_t < k);
            jumped = true;
        } else {
            prev[cur_pos] = cur_t;
        }
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
