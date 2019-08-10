fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let p: usize = sc.read();
    let a: Vec<usize> = sc.vec(p);
    let comb = Combination::new(p, p);

    // 1 - (x-j)^(p-1)
    let mut ans = vec![0; p];
    for i in 0..p {
        // for x^i
        let mut sum = 0;
        for j in 0..p {
            let a = a[j];
            let c = comb.get(p - 1, i);
            let j = mod_pow(j, p - 1 - i, p);
            sum += a * c * j;
            sum %= p;
        }
        if (p - 1 - i + 1) % 2 != 0 {
            sum = (p - sum) % p;
        }
        if i == 0 {
            for j in 0..p {
                sum += a[j];
            }
            sum %= p;
        }
        ans[i] = sum;
    }
    for i in 0..p {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
}

fn mod_pow(mut cur: usize, mut e: usize, p: usize) -> usize {
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % p;
        }
        cur = (cur * cur) % p;
        e >>= 1;
    }
    result
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
