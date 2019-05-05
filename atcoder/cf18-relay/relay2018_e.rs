const MOD: usize = 1e9 as usize + 7;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };

    let s = sc.chars();
    let num_w: usize = sc.read();
    let n = s.len();
    let num_f = n - num_w;

    let num_w = num_w - s.iter().filter(|&&c| c == 'W').count();
    let num_f = num_f - s.iter().filter(|&&c| c == 'F').count();
    let comb = Combination::new(n, MOD);

    let mut ans = 0;
    for i in 0..n {
        let c1 = s[i];
        let c2 = s[(i + 1) % n];

        if c1 != 'F' && c2 != 'W' && (num_w != 0 || c1 != '?') && (num_f != 0 || c2 != '?') {
            let x = if c1 == '?' { num_w - 1 } else { num_w };
            let y = if c2 == '?' { num_f - 1 } else { num_f };
            ans += comb.get(x + y, x);
        }
        if c1 != 'W' && c2 != 'F' && (num_f != 0 || c1 != '?') && (num_w != 0 || c2 != '?') {
            let x = if c1 == '?' { num_f - 1 } else { num_f };
            let y = if c2 == '?' { num_w - 1 } else { num_w };
            ans += comb.get(x + y, x);
        }
    }

    println!("{}", ans % MOD);
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

    pub fn h(&self, n: usize, r: usize) -> usize {
        self.get(n + r - 1, r)
    }
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
