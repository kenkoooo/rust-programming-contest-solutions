const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let k: usize = sc.read();
    let n: usize = sc.read();

    let combination = Combination::new(k + n, MOD);
    for sum in 2..(2 * k + 1) {
        println!("{}", solve(sum, k, n, &combination));
    }
}

fn solve(sum: usize, max_value: usize, num_dice: usize, combination: &Combination) -> usize {
    let mut pairs = 0;
    for i in 1.. {
        if i * 2 > sum {
            break;
        }
        let other = sum - i;
        if other > max_value {
            continue;
        }
        pairs += 1;
    }

    let mut ans = combination.h(max_value, num_dice);
    for used_pairs in 1..(pairs + 1) {
        if num_dice < used_pairs * 2 {
            break;
        }

        let num_other_dice = num_dice - used_pairs * 2;
        let mut invalid_moves = combination.get(num_other_dice + max_value - 1, num_other_dice)
            * combination.get(pairs, used_pairs);
        invalid_moves %= MOD;
        if used_pairs % 2 == 1 {
            ans += MOD - invalid_moves;
        } else {
            ans += invalid_moves;
        }
        ans %= MOD;
    }
    ans
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
