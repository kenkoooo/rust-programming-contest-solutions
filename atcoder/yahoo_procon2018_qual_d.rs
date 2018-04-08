use std::cmp;

const MOD: usize = 1_000_000_007;
const UPPER_A: usize = 4096;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let x: usize = sc.read();
    let y: usize = sc.read();
    let z = x ^ y;
    let array: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    let b: Vec<Vec<usize>> = (0..k).map(|_| {
        (0..k).map(|_| sc.read::<usize>() ^ x).collect()
    }).collect();

    // check
    for i in 0..k {
        if b[i][i] != 0 && b[i][i] != z {
            println!("0");
            return;
        }
        for j in 0..k {
            if b[i][j] != b[j][i] && b[i][j] != b[j][i] ^ z {
                println!("0");
                return;
            }
        }
    }

    let mut count = vec![0; UPPER_A];
    let mut xor_count = vec![0; UPPER_A];
    for &a in &array {
        count[cmp::min(a, a ^ z)] += 1;
        if a ^ z < a {
            xor_count[a ^ z] += 1;
        }
    }

    let mut ans = 0;
    let comb = Combination::new(UPPER_A, MOD);
    for a0 in 0..UPPER_A {
        if count[a0] == 0 { continue; }

        let mut can_construct = true;
        let mut used_count = vec![0; UPPER_A];
        used_count[a0] += 1;
        for i in 1..k {
            let ai = cmp::min(b[0][i] ^ a0, b[0][i] ^ a0 ^ z);
            if used_count[ai] == count[ai] {
                can_construct = false;
                break;
            }
            used_count[ai] += 1;
        }

        if !can_construct { continue; }

        let mut ans_for_a0 = 1;
        for i in 0..UPPER_A {
            if used_count[i] == 0 { continue; }

            let needed = used_count[i];
            let xor = xor_count[i];
            let non_xor = count[i] - xor;

            let max_xor = cmp::min(needed, xor_count[i]);
            let min_xor = if needed < non_xor { 0 } else { needed - non_xor };

            let mut p = 0;
            for choose in min_xor..(max_xor + 1) {
                p += comb.get(needed, choose);
                if p > MOD { p -= MOD; }
            }
            ans_for_a0 *= p;
            ans_for_a0 %= MOD;
        }
        ans += ans_for_a0;
        if ans > MOD { ans -= MOD; }
    }
    println!("{}", ans);
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
        for i in 0..max { fact[i + 1] = fact[i] * (i + 1) % modulo; }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination { fact: fact, inv_fact: inv_fact, modulo: modulo }
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
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

