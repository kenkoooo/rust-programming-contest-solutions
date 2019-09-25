fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let s = sc.chars();

    let mut d_sum = vec![0; n + 1];
    let mut m_sum = vec![0; n + 1];
    let mut c_sum = vec![0; n + 1];
    for i in 0..n {
        d_sum[i + 1] = d_sum[i] + if s[i] == 'D' { 1 } else { 0 };
        m_sum[i + 1] = m_sum[i] + if s[i] == 'M' { 1 } else { 0 };
        c_sum[i + 1] = c_sum[i] + if s[i] == 'C' { 1 } else { 0 };
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let k: usize = sc.read();
        let mut d_table: Vec<i64> = vec![0; n + 1];
        let mut d_remove: Vec<i64> = vec![0; n + 1];
        for i in 0..n {
            if s[i] == 'D' {
                d_table[i] += 1;
                if i + k <= n {
                    d_table[i + k] -= 1;
                    d_remove[i + k] += 1;
                }
            }
        }
        let mut d_acc = vec![0; n + 1];
        for i in 0..n {
            d_acc[i] = d_table[i];
            if i > 0 {
                d_acc[i] += d_acc[i - 1];
            }
        }
        let mut m_table = vec![0; n + 1];
        for i in 1..n {
            m_table[i] = m_table[i - 1];
            if s[i] == 'M' {
                m_table[i] += d_acc[i - 1];
            }
            let range_sum = m_sum[i + 1] - if i + 1 >= k { m_sum[i + 1 - k] } else { 0 };
            m_table[i] -= d_remove[i] * range_sum;
        }

        let mut ans = 0;

        //        eprintln!("k={}", k);
        //        format(&s);
        //        format(&m_table);
        //        format(&d_table);

        for i in 1..n {
            if s[i] == 'C' {
                //                let mut naive_sum = 0;
                //                let t = if i < (k - 1) { 0 } else { i - (k - 1) };
                //                for a in t..i {
                //                    for b in (a + 1)..i {
                //                        let c = i;
                //                        if a < b && b < c && c - a < k && s[a] == 'D' && s[b] == 'M' && s[c] == 'C'
                //                        {
                //                            naive_sum += 1;
                //                        }
                //                    }
                //                }
                //                assert_eq!(naive_sum, m_table[i], "i={}", i);

                ans += m_table[i];
            }
        }

        println!("{}", ans);
    }
}

fn format<T: std::fmt::Display>(x: &[T]) {
    for t in x.iter() {
        print!("{}\t", t);
    }
    println!();
}
/*
7
DMCDMCD
1
4
*/
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
