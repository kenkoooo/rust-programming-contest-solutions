use std::cmp;
const INF: i64 = 1e16 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let mut abc: Vec<i64> = sc.vec(3);
    abc.sort();

    let l: Vec<i64> = sc.vec(n);
    let sums = (0..(1 << n))
        .map(|mask| {
            l.iter()
                .enumerate()
                .filter(|&(i, _)| (1 << i) & mask != 0)
                .map(|(_, &l)| l)
                .sum::<i64>()
        })
        .collect::<Vec<_>>();

    let mut ans = INF;
    for mask1 in 1..(1 << n) {
        for mask2 in 1..(1 << n) {
            if mask1 & mask2 != 0 {
                continue;
            }
            for mask3 in 1..(1 << n) {
                if mask1 & mask3 != 0 || mask2 & mask3 != 0 {
                    continue;
                }
                let mut v = vec![sums[mask1], sums[mask2], sums[mask3]];
                let additional_costs =
                    (mask1.count_ones() + mask2.count_ones() + mask3.count_ones() - 3) * 10;
                v.sort();
                let cost = (v[0] - abc[0]).abs() + (v[1] - abc[1]).abs() + (v[2] - abc[2]).abs();
                ans = cmp::min(ans, cost + (additional_costs as i64));
            }
        }
    }
    println!("{}", ans);
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
