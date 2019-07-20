fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let m = sc.read();
    let mut a: Vec<usize> = sc.vec(n);
    let b: Vec<usize> = sc.vec(m);

    let max_b: usize = *b.iter().max().unwrap();
    let mut count_b = vec![0; max_b + 1];
    for b in b.into_iter() {
        count_b[b] += 1;
    }
    let count_b: Vec<(usize, usize)> = count_b
        .into_iter()
        .enumerate()
        .filter(|&(_, c)| c > 0)
        .collect();

    a.sort();

    let mut sum_a = vec![0; n + 1];
    for i in 0..n {
        sum_a[i + 1] = sum_a[i] + a[i];
    }

    let mut sum = 0;
    for (b, count) in count_b.into_iter() {
        let mut head = 0;
        let mut cur_b = b;
        let mut ans = 0;
        while head < a.len() {
            let upper = a
                .binary_search_by_key(&(cur_b * 2 - 1), |&x| x * 2)
                .err()
                .unwrap();

            let count = upper - head;
            ans += sum_a[upper] - sum_a[head];
            ans -= (cur_b - b) * count;

            head = upper;
            cur_b += b;
        }

        sum += count * ans;
    }

    println!("{}", sum);
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
