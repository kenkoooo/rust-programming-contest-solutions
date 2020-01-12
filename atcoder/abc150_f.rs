const MOD: u64 = 998244353;
fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let b: Vec<u64> = sc.vec(n);

    let mut a_hash: Vec<Vec<RollingHash>> = vec![vec![], vec![]];
    let mut b_hash = vec![];
    for d in 0..31 {
        for xor in 0..2 {
            let mut a = a
                .iter()
                .map(|&a| (((a >> d) & 1) ^ xor) as u8)
                .collect::<Vec<_>>();
            a.extend(a.clone());
            let a = RollingHash::new(&a, MOD);
            a_hash[xor as usize].push(a);
        }
        let b = b.iter().map(|&b| ((b >> d) & 1) as u8).collect::<Vec<_>>();
        let h = RollingHash::new(&b, MOD);
        b_hash.push(h.get_hash(0, n));
    }

    for k in 0..n {
        let mut x = vec![];
        for digit in 0..31 {
            let b = b_hash[digit];
            let mut xx = vec![];
            for xor in 0..2 {
                let a = a_hash[xor][digit].get_hash(k, k + n);
                if a == b {
                    xx.push(xor);
                }
            }
            if xx.is_empty() {
                break;
            }
            x.push(xx);
        }
        if x.len() != 31 {
            continue;
        }
        x.reverse();
        let mut ans = vec![0];
        for x in x.into_iter() {
            let mut next = vec![];
            for ans in ans.into_iter() {
                for &x in x.iter() {
                    let sub = (ans << 1) + x;
                    next.push(sub);
                }
            }
            ans = next;
        }

        for ans in ans.into_iter() {
            sc.write(format!("{} {}\n", k, ans));
        }
    }
}

pub struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl RollingHash {
    pub fn new(s: &[u8], base: u64) -> RollingHash {
        let n = s.len();
        let mut hash: Vec<u64> = vec![0; n + 1];
        let mut pow: Vec<u64> = vec![0; n + 1];
        pow[0] = 1;
        for i in 0..n {
            pow[i + 1] = pow[i].wrapping_mul(base);
            hash[i + 1] = hash[i].wrapping_mul(base).wrapping_add(s[i] as u64);
        }
        RollingHash {
            hash: hash,
            pow: pow,
        }
    }

    /// Get hash of [l, r)
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        self.hash[r].wrapping_sub(self.hash[l].wrapping_mul(self.pow[r - l]))
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
