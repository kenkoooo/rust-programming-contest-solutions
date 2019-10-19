fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let xy: Vec<Vec<bool>> = (0..n)
        .map(|_| sc.chars().into_iter().map(|c| c == '1').collect())
        .collect::<Vec<_>>();
    let xz: Vec<Vec<bool>> = (0..n)
        .map(|_| sc.chars().into_iter().map(|c| c == '1').collect())
        .collect::<Vec<_>>();

    let mut yz: Vec<Vec<Option<bool>>> = vec![vec![None; n]; n];
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if xy[x][y] && !xz[x][z] {
                    match yz[y][z] {
                        Some(yz) => {
                            if yz {
                                println!("-1");
                                return;
                            }
                        }
                        None => {
                            yz[y][z] = Some(false);
                        }
                    }
                }
            }
        }
    }

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if xy[x][y] && xz[x][z] {
                    if yz[y][z].is_none() {
                        yz[y][z] = Some(true);
                    }
                }
            }
        }
    }

    let mut reachable = vec![vec![false; n]; n];
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                match yz[y][z] {
                    Some(yz) => {
                        if xy[x][y] && yz {
                            reachable[x][z] = true;
                        }
                    }
                    None => {
                        if xy[x][y] {
                            reachable[x][z] = true;
                        }
                    }
                }
            }
        }
    }

    for x in 0..n {
        for z in 0..n {
            if xz[x][z] != reachable[x][z] {
                println!("-1");
                return;
            }
        }
    }

    for y in 0..n {
        for z in 0..n {
            match yz[y][z] {
                Some(yz) => {
                    if yz {
                        print!("1");
                    } else {
                        print!("0");
                    }
                }
                None => {
                    print!("1");
                }
            }
        }
        println!();
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
