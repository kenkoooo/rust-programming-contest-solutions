use std::cmp;
const MOD: usize = 1_000_000_007;
const MAX_A: usize = 2048;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let x: usize = sc.read();
    let y: usize = sc.read();

    let z = x ^ y;

    let a: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    let b: Vec<Vec<usize>> = (0..k)
        .map(|_| (0..k).map(|_| sc.read::<usize>() ^ x).collect())
        .collect();

    println!("{}", solve(&a, &b, z));
}

fn solve(a: &Vec<usize>, b: &Vec<Vec<usize>>, z: usize) -> usize {
    let combination = Combination::new(MAX_A + 1, MOD);
    let k = b.len();
    for i in 0..k {
        if b[i][i] != 0 && b[i][i] != z {
            return 0;
        }
        for j in 0..k {
            if b[i][j] != b[j][i] && b[i][j] != b[j][i] ^ z {
                return 0;
            }
        }
    }

    let mut count = vec![0; (MAX_A + 1)];
    let mut xor_count = vec![0; (MAX_A + 1)];
    for &a in a {
        count[cmp::min(a, a ^ z)] += 1;
        if (a ^ z) < a {
            xor_count[a ^ z] += 1;
        }
    }

    let mut ans = 0;
    for a0 in 0..(MAX_A + 1) {
        if count[a0] == 0 {
            continue;
        }

        let mut possible = true;
        let mut used = vec![0; (MAX_A + 1)];
        used[a0] = 1;
        for j in 1..k {
            let aj = cmp::min(b[0][j] ^ a0, b[0][j] ^ a0 ^ z);
            if used[aj] >= count[aj] {
                possible = false;
                break;
            }
            used[aj] += 1;
        }

        if !possible {
            continue;
        }

        let mut ans_a0 = 1;
        for a in 0..(MAX_A + 1) {
            if used[a] == 0 {
                continue;
            }

            let mut partial_ans = 0;

            let used = used[a];
            let count = count[a];
            let xor = xor_count[a];
            let non_xor = count - xor;
            let min_xor = if used >= non_xor { used - non_xor } else { 0 };
            let max_xor = cmp::min(xor, used);
            for xor in min_xor..(max_xor + 1) {
                partial_ans += combination.get(used, xor);
                if partial_ans >= MOD {
                    partial_ans -= MOD;
                }
            }

            ans_a0 *= partial_ans;
            ans_a0 %= MOD;
        }
        ans += ans_a0;
        if ans >= MOD {
            ans -= MOD;
        }
    }

    return ans;
}
pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Combination {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination {
            fact: fact,
            inv_fact: inv_fact,
            modulo: modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x >= y);
        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }
}
struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
