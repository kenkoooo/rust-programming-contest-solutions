fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let l: u64 = sc.read();
    let t: u64 = sc.read();
    let xw: Vec<_> = (0..n)
        .map(|_| (sc.read::<u64>(), sc.read::<u32>() == 1))
        .collect();

    let n = n as u64;
    let mut clock_count = 0;
    let mut counter_count = 0;
    for &(x, clockwise) in xw.iter() {
        if clockwise {
            clock_count += (x + t) / l;
        } else {
            counter_count += (l - x + t - 1) / l;
        }
    }

    clock_count %= n;
    counter_count %= n;
    let mut end_state = xw
        .iter()
        .map(|&(x, clockwise)| {
            if clockwise {
                ((x + t) % l, clockwise)
            } else {
                ((l - (l - x + t) % l) % l, clockwise)
            }
        })
        .collect::<Vec<_>>();
    end_state.sort();
    let first = (clock_count - counter_count + n) % n;
    for i in 0..n {
        println!("{}", end_state[((first + i) % n) as usize].0);
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
