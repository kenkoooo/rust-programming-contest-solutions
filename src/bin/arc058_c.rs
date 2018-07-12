const MOD: usize = 1_000_000_007;
const MAX: usize = 1 << 17;

fn mod_pow(x: usize, e: usize) -> usize {
    let mut cur = x;
    let mut res = 1;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            res = (res * cur) % MOD;
        }
        e >>= 1;
        cur = (cur * cur) % MOD;
    }
    res
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let x = sc.usize_read();
    let y = sc.usize_read();
    let z = sc.usize_read();
    let v_xyz = vec![x, y, z];

    let xyz = x + y + z;
    let mut dp = vec![0; MAX];
    dp[0] = 1;
    for i in 0..n {
        let mut next = vec![0; MAX];
        for mask in 0..MAX {
            if dp[mask] == 0 {
                continue;
            }
            let mut prev_numbers = mask2nums(mask);

            for a in 1..11 {
                let mut check = false;
                let mut tmp_sum = 0;
                let mut cur_pos = 2;
                prev_numbers.push(a);
                for &prev in prev_numbers.iter().rev() {
                    tmp_sum += prev;
                    if tmp_sum == v_xyz[cur_pos] {
                        if cur_pos > 0 {
                            cur_pos -= 1;
                            tmp_sum = 0;
                        } else {
                            check = true;
                            break;
                        }
                    } else if tmp_sum > v_xyz[cur_pos] {
                        break;
                    }
                }

                if !check {
                    let mut next_mask = 0;
                    for &prev in &prev_numbers {
                        next_mask <<= prev;
                        next_mask += (1 << (prev - 1));
                    }
                    next_mask &= (1 << xyz) - 1;
                    next[next_mask] = (next[next_mask] + dp[mask]) % MOD;
                }
                prev_numbers.pop();
            }
        }
        dp = next;
    }

    let mut sum = 0;
    for &dp in &dp {
        sum = (sum + dp) % MOD;
    }
    let ans = mod_pow(10, n) + MOD - sum;
    println!("{}", ans % MOD);
}

fn mask2nums(mask: usize) -> Vec<usize> {
    let mut prev_numbers = vec![];
    let mut mask = mask;
    while mask > 0 {
        let zeros = trailing_zeros(mask);
        prev_numbers.push(zeros + 1);
        mask >>= zeros + 1;
    }
    prev_numbers.reverse();
    prev_numbers
}

fn trailing_zeros(x: usize) -> usize {
    assert!(x > 0);
    for i in 0..64 {
        if (x >> i) & 1 != 0 {
            return i;
        }
    }
    panic!();
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
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

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
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
