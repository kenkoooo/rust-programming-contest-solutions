fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let length: usize = sc.read();
    let time: usize = sc.read();
    let xw: Vec<(usize, bool)> = (0..n).map(|_| (sc.read(), sc.read::<i32>() == 1)).collect();

    let (first_pos, first_clock) = xw[0];
    let mut cross_count = 0;
    for &(pos, clock) in &xw[1..n] {
        if clock == first_clock {
            continue;
        }
        let initial_distance = if first_clock {
            pos - first_pos
        } else {
            length + first_pos - pos
        };
        if time * 2 < initial_distance {
            continue;
        }

        cross_count += (time * 2 - initial_distance + length - 1) / length;
    }

    let final_num = if first_clock {
        cross_count % n
    } else {
        (n - cross_count % n) % n
    };

    let final_pos = if first_clock {
        (first_pos + time) % length
    } else {
        (first_pos + length - time % length) % length
    };

    let mut final_states = xw.iter()
        .map(|&(pos, clock)| {
            if clock {
                (pos + time) % length
            } else {
                (pos + length - time % length) % length
            }
        })
        .collect::<Vec<_>>();
    final_states.sort();

    let mut final_i = 0;
    for i in 0..n {
        if final_states[i] == final_pos {
            if first_clock || final_states[i] != final_states[(i + 1) % n] {
                final_i = i;
            } else {
                final_i = i + 1;
            }
            break;
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[(final_num + i) % n] = final_states[(final_i + i) % n];
    }

    for &a in &ans {
        println!("{}", a);
    }
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
