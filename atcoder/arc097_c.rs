use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut black_place = vec![0; n];
    let mut white_place = vec![0; n];
    for i in 0..(2 * n) {
        let c = sc.read::<String>();
        let a: usize = sc.read::<usize>() - 1;
        if c == "W" {
            white_place[a] = i;
        } else {
            black_place[a] = i;
        }
    }

    let back = |place: &Vec<usize>| {
        let mut min_back = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if place[i] < place[j] {
                    min_back[i] += 1;
                }
            }
        }
        min_back
    };

    let min_back_w = back(&white_place);
    let min_back_b = back(&black_place);

    let calc_cost = |place: &Vec<usize>, min_back: &Vec<usize>, other_place: &Vec<usize>| {
        let mut cost = vec![vec![0; n + 1]; n];
        for a in 0..n {
            let back = min_back[a];
            let place = place[a];
            cost[a][0] = back;
            for b in 0..n {
                cost[a][b + 1] = cost[a][b];
                if other_place[b] > place {
                    cost[a][b + 1] += 1;
                }
            }
        }
        cost
    };

    let cost_w = calc_cost(&white_place, &min_back_w, &black_place);
    let cost_b = calc_cost(&black_place, &min_back_b, &white_place);

    let mut dp = vec![vec![std::usize::MAX; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..(n + 1) {
        for j in 0..(n + 1) {
            if i < n {
                let white_place = white_place[i] + cost_w[i][j] - (i + j);
                dp[i + 1][j] = cmp::min(dp[i + 1][j], dp[i][j] + white_place);
            }

            if j < n {
                let black_place = black_place[j] + cost_b[j][i] - (i + j);
                dp[i][j + 1] = cmp::min(dp[i][j + 1], dp[i][j] + black_place);
            }
        }
    }
    println!("{}", dp[n][n]);
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
