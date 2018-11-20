use std::cmp;

fn solve(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let n = a.len();
    let mut v = vec![];
    for i in 0..n {
        v.push((a[i], i));
        v.push((b[i], i));
    }
    v.sort();

    let mut ans = 0;
    let mut used_count = vec![0; n];
    for i in 0..n {
        let (t, i) = v[i];
        used_count[i] += 1;
        ans += t;
    }

    let used_two = used_count.iter().any(|&c| c == 2);
    if used_two {
        return ans;
    }

    let mut min = 1e15 as usize;
    for i in 0..n {
        let (t, i) = v[i];
        let ans = ans - t;
        let (t, j) = v[n];
        if i == j {
            let (t, _) = v[n + 1];
            min = cmp::min(ans + t, min);
        } else {
            min = cmp::min(ans + t, min);
        }
    }
    min
}

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let mut a: Vec<usize> = vec![0; n];
    let mut b: Vec<usize> = vec![0; n];
    for i in 0..n {
        a[i] = sc.read();
        b[i] = sc.read();
    }

    let all_a: usize = a.iter().sum();
    let all_b: usize = b.iter().sum();
    println!("{}", cmp::min(solve(&a, &b), cmp::min(all_a, all_b)));
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
