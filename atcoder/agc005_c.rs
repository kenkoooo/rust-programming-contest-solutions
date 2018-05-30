use std::cmp;
fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let max: usize = a.iter().map(|&i| i).max().unwrap();

    if n > 2 && max == 1 {
        println!("Impossible");
        return;
    }

    let mut count = vec![0; max + 1];
    for &a in &a {
        count[a] += 1;
    }

    for i in 0..(max + 1) {
        if i < (max + 1) / 2 {
            if count[i] > 0 {
                println!("Impossible");
                return;
            }
        } else {
            if i * 2 != max && count[i] < 2 {
                println!("Impossible");
                return;
            } else if i * 2 == max && count[i] != 1 {
                println!("Impossible");
                return;
            } else if max % 2 == 1 && i == (max + 1) / 2 && count[i] != 2 {
                println!("Impossible");
                return;
            }
        }
    }

    // let mut tree = vec![vec![]; n];
    // {
    //     let mut count = count.clone();
    //     for i in 0..max {
    //         tree[i].push(i + 1);
    //         tree[i + 1].push(i);
    //     }
    //     for i in 0..(max + 1) {
    //         count[max_dist(&tree, i, n, 0)] -= 1;
    //     }

    //     let mut t = max + 1;
    //     for d in 0..(max + 1) {
    //         for _ in 0..count[d] {
    //             tree[d - 1].push(t);
    //             tree[t].push(d - 1);
    //             t += 1;
    //         }
    //     }
    //     assert_eq!(t, n);
    // }
    // {
    //     let mut check = vec![0; max + 1];
    //     for i in 0..n {
    //         check[max_dist(&tree, i, n, 0)] += 1;
    //     }

    //     for i in 0..(max + 1) {
    //         assert_eq!(count[i], check[i]);
    //     }
    // }

    println!("Possible");
}

fn max_dist(tree: &Vec<Vec<usize>>, v: usize, p: usize, depth: usize) -> usize {
    if tree[v].len() == 1 && tree[v][0] == p {
        return depth;
    }
    let mut max = 0;
    for &to in &tree[v] {
        if to == p {
            continue;
        }
        max = cmp::max(max_dist(tree, to, v, depth + 1), max);
    }
    return max;
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
