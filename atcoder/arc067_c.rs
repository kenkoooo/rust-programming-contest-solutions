const MOD: usize = 1_000_000_007;

fn main() {
    let combination = Combination::new(1000000, MOD);

    let mut sc = Scanner::new();
    let (n, a, b, c, d): (usize, usize, usize, usize, usize) =
        (sc.read(), sc.read(), sc.read(), sc.read(), sc.read());
    let mut dp = vec![vec![0; b + 2]; n + 1];
    dp[0][a] = 1;

    for person in 0..(n + 1) {
        for upper_size in a..(b + 1) {
            if dp[person][upper_size] == 0 {
                continue;
            }

            dp[person][upper_size + 1] += dp[person][upper_size];
            dp[person][upper_size + 1] %= MOD;

            let group_size = upper_size;
            let mut cur = dp[person][upper_size];
            for new_group_num in 1..(d + 1) {
                let person = person + new_group_num * group_size;
                if person > n {
                    break;
                }

                let selectable = n - person + group_size;
                cur *= combination.get(selectable, group_size);
                cur %= MOD;
                if new_group_num < c {
                    continue;
                }

                dp[person][group_size + 1] += (cur * combination.inv_fact[new_group_num]) % MOD;
                dp[person][group_size + 1] %= MOD;
            }
        }
    }

    println!("{}", dp[n][b + 1]);
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
