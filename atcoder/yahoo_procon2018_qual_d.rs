use std::cmp;

const MAX_A: usize = 2048;
const MOD: usize = 1000000007;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let x: usize = sc.read();
    let y: usize = sc.read();
    let c: Vec<usize> = (0..n).map(|_| sc.read()).collect();

    let mut a = vec![vec![0; k]; k];
    for i in 0..k {
        for j in 0..k {
            a[i][j] = sc.read::<usize>() ^ x;
        }
    }

    println!("{}", solve(k, x ^ y, &c, &a));
}

fn solve(k: usize, z: usize, x: &Vec<usize>, b: &Vec<Vec<usize>>) -> usize {
    let comb = Combination::new(MAX_A * 2, MOD);

    // check
    for i in 0..k {
        if b[i][i] != 0 && b[i][i] != z { return 0; }
    }
    for i in 0..k {
        for j in 0..k {
            let delta = b[i][j] ^ b[j][i];
            if delta != 0 && delta != z { return 0; }
        }
    }

    let mut count = vec![0; MAX_A + 1];
    let mut xor_counted = vec![0; MAX_A + 1];
    for xi in x {
        let xi = *xi;
        count[cmp::min(xi, xi ^ z)] += 1;
        if (xi ^ z) < xi { xor_counted[xi ^ z] += 1; }
    }

    let mut answer = 0;
    for a0 in 0..(MAX_A + 1) {
        if count[a0] == 0 { continue; }
        let mut used = vec![0; MAX_A + 1];
        used[a0] += 1;

        let mut can_construct = true;
        for j in 1..k {
            let aj = cmp::min(b[0][j] ^ a0, b[0][j] ^ a0 ^ z);

            if used[aj] == count[aj] {
                can_construct = false;
                break;
            }
            used[aj] += 1;
        }

        if !can_construct { continue; }

        let mut ans_for_a0 = 1;
        for a in 0..(MAX_A + 1) {
            if used[a] == 0 { continue; }

            let needed = used[a];
            let max_not_xor = cmp::min(count[a] - xor_counted[a], needed);
            let min_not_xor = if needed < xor_counted[a] { 0 } else { needed - xor_counted[a] };

            let mut combination_sum = 0;
            for choose in min_not_xor..(max_not_xor + 1) {
                combination_sum += comb.get(needed, choose);
                if combination_sum > MOD { combination_sum -= MOD; }
            }
            ans_for_a0 *= combination_sum;
            ans_for_a0 %= MOD;
        }
        answer += ans_for_a0;
        if answer > MOD { answer -= MOD; }
    }

    return answer;
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

