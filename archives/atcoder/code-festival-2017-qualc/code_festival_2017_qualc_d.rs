use std::cmp;

const INF: u32 = 1 << 30;

fn main() {
    let stdin = std::io::stdin();
    let mut sc = scanner::Scanner::new(stdin.lock());
    let s: Vec<u32> = sc
        .read::<String>()
        .chars()
        .map(|c| c as u32 - 'a' as u32)
        .collect();
    let n = s.len();
    let mut hash: Vec<u32> = vec![0; n + 1];
    for i in 0..n {
        hash[i + 1] = hash[i] ^ (1 << s[i]);
    }

    let mut prefix: Vec<u32> = vec![0];
    for i in 0..26 {
        prefix.push(1 << i);
    }

    let mut opt = vec![INF; n + 1];
    let mut dp = vec![INF; 1 << 26];
    opt[0] = 0;
    dp[0] = 0;

    for i in 0..n {
        let hash = hash[i + 1];
        for &prefix in prefix.iter() {
            let suffix = hash ^ prefix;
            opt[i + 1] = cmp::min(opt[i + 1], dp[suffix as usize] + 1);
        }
        dp[hash as usize] = cmp::min(dp[hash as usize], opt[i + 1]);
    }

    println!("{}", opt[n]);
}

pub mod scanner {
    use std::fmt::Debug;
    use std::io::Read;
    use std::str::{self, FromStr};

    pub struct Scanner<R: Read> {
        reader: R,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner { reader: reader }
        }

        pub fn read<T>(&mut self) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            let buf: Vec<u8> = self
                .reader
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| (b as char).is_whitespace())
                .take_while(|&b| !(b as char).is_whitespace())
                .fold(Vec::new(), |mut buf, c| {
                    buf.push(c);
                    buf
                });

            unsafe { str::from_utf8_unchecked(&buf) }
                .parse()
                .expect("Parse error.")
        }
    }
}
