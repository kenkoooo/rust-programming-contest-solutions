fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let k: usize = sc.read();
    let power: Vec<i64> = sc.vec(n);
    if solve(&power, k) {
        println!("Yes");
    } else {
        println!("No");
    }

    // i <= hit
    // (K-(hit-i))**2
    // (K-hit+i)**2
    // i**2 + 2(K-hit)i + (K-hit)**2

    // i > hit
    // (K - (i-hit))**2
    // (K - i + hit)**2
    // (i - (hit+K))**2
    // i**2 -2(hit+K)i + (hit+K)**2
}

fn solve(power: &Vec<i64>, k: usize) -> bool {
    let n = power.len();
    let mut acc_a = vec![0; n + 1];
    let mut acc_b = vec![0; n + 1];
    let mut acc_c = vec![0; n + 1];
    for i in 0..n {
        let pi = i as i64;
        let damage = acc_a[i] * pi * pi + acc_b[i] * pi + acc_c[i];
        if damage > power[i] {
            return false;
        }

        if i + k + k - 1 > n && damage != power[i] {
            return false;
        }
        if damage < power[i] {
            let remain = power[i] - damage;
            let hit = i + k - 1;

            acc_a[i] += remain;
            acc_a[hit + k] -= remain;

            let x = hit as i64 - k as i64;
            let y = k as i64 + hit as i64;

            acc_b[i] += -remain * 2 * x;
            acc_b[hit + 1] -= -remain * 2 * x;
            acc_b[hit + 1] += -remain * 2 * y;
            acc_b[hit + k] -= -remain * 2 * y;

            acc_c[i] += remain * x * x;
            acc_c[hit + 1] -= remain * x * x;
            acc_c[hit + 1] += remain * y * y;
            acc_c[hit + k] -= remain * y * y;

            if acc_b[hit + 1].abs() > 1e15 as i64
                || acc_b[hit + k].abs() > 1e15 as i64
                || acc_c[hit + 1].abs() > 1e15 as i64
                || acc_c[hit + k].abs() > 1e15 as i64
            {
                return false;
            }
        }

        acc_a[i + 1] += acc_a[i];
        acc_b[i + 1] += acc_b[i];
        acc_c[i + 1] += acc_c[i];
    }
    true
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
