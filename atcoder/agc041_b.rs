use std::collections::BTreeMap;

const INF: usize = 1e15 as usize;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let vote: usize = sc.read();
    let p: usize = sc.read();

    let mut count = BTreeMap::new();
    for _ in 0..n {
        let a: usize = sc.read();
        *count.entry(a).or_insert(0) += 1;
    }

    let mut p_th = INF;
    {
        let mut c = 0;
        for (&a, &count) in count.iter().rev() {
            c += count;
            if c >= p {
                p_th = a;
                break;
            }
        }
    }
    assert_ne!(p_th, INF);
    //    eprintln!("{}", p_th);

    let mut ans = 0;
    let mut ceil_count = 0;
    let mut sum = 0;
    let mut sum_count = 0;
    for (a, count) in count.into_iter().rev() {
        //        println!(
        //            "a={} count={} ceil_count={} sum={} sum_count={}",
        //            a, count, ceil_count, sum, sum_count
        //        );
        if a + m < p_th {
            break;
        }
        if ceil_count + count <= p - 1 {
            ans += count;
            ceil_count += count;
        } else if ceil_count < p - 1 {
            let space = p - 1 - ceil_count;
            ceil_count += space;
            let remain = count - space;
            assert!(remain > 0);
            sum += remain * a;
            sum_count += remain;
            ans += count;
        } else {
            assert_eq!(ceil_count, p - 1);
            let already = n - sum_count;
            if already >= vote {
                ans += count;
            } else {
                let remain_vote = vote - already;
                let remaining = remain_vote * m;
                let height = a + m;
                let rectangle = height * sum_count;
                if remaining + sum <= rectangle {
                    ans += count;
                }
            }
            sum_count += count;
            sum += a * count;
        }
    }

    println!("{}", ans);
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
