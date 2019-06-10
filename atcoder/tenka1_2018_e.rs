use std::cmp;
use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let h: usize = sc.read();
    let w: usize = sc.read();
    let mut board = vec![vec![false; w]; h];
    for i in 0..h {
        let s = sc.chars();
        for j in 0..w {
            board[i][j] = s[j] == '#';
        }
    }

    let mut inv_board = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            inv_board[i][j] = board[h - 1 - i][j];
        }
    }

    let ans1 = solve(&board);
    let ans2 = solve(&inv_board);

    let mut ans = ans1 + ans2;
    let mut map = BTreeMap::new();
    for x in 0..h {
        for y in 0..w {
            if board[x][y] {
                // y = -x + l
                let l = x + y;
                map.entry(l).or_insert(Vec::new()).push((x, y));
            }
        }
    }

    for v in map.values() {
        for j in 0..v.len() {
            for i in 0..j {
                let (x1, y1) = v[i];
                let (x2, y2) = v[j];
                let d = y1 - y2;

                let mut c = vec![];
                if x1 >= d {
                    c.push((x1 - d, y2));
                }
                if y2 >= d {
                    c.push((x1, y2 - d));
                }
                if y1 + d < w {
                    c.push((x2, y1 + d));
                }
                if x2 + d < h {
                    c.push((x2 + d, y1));
                }

                for (x3, y3) in c.into_iter() {
                    if board[x3][y3] {
                        ans -= 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

fn solve(board: &Vec<Vec<bool>>) -> usize {
    let h = board.len();
    let w = board[0].len();
    let mut map = BTreeMap::new();
    for x in 0..h {
        for y in 0..w {
            if board[x][y] {
                // y = -x + l
                let l = x + y;
                map.entry(l).or_insert(Vec::new()).push((x, y));
            }
        }
    }

    let sum: BTreeMap<usize, Vec<usize>> = map
        .iter()
        .map(|(&l, v)| {
            let mut board = vec![0; h];
            for &(x, _) in v.iter() {
                board[x] = 1;
            }
            let mut sum = vec![0; h + 1];
            for i in 0..h {
                sum[i + 1] = sum[i] + board[i];
            }
            (l, sum)
        })
        .collect();

    let mut ans = 0;
    for (&l, v) in map.iter() {
        for j in 0..v.len() {
            for i in 0..j {
                let (x1, y1) = v[i];
                let (x2, y2) = v[j];
                assert_eq!(x2 - x1, y1 - y2);
                let d = x2 - x1;

                if l >= 2 * d {
                    let lower = if x1 >= d { x1 - d } else { 0 };
                    let upper = x1 + 1;
                    let l3 = l - 2 * d;
                    if let Some(sum) = sum.get(&l3) {
                        ans += sum[upper] - sum[lower];
                    }
                }

                let lower = x2;
                let upper = cmp::min(x2 + d + 1, h);
                let l3 = l + 2 * d;
                if let Some(sum) = sum.get(&l3) {
                    ans += sum[upper] - sum[lower];
                }
            }
        }
    }
    ans
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
