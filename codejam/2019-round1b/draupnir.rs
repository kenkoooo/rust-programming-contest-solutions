fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let t: usize = sc.read();
    let _: usize = sc.read();
    for _ in 0..t {
        println!("200");
        let day200: u64 = sc.read();
        let ring4 = day200 / (1 << 50);
        let ring5 = (day200 - ring4 * (1 << 50)) / (1 << 40);
        let ring6 = (day200 - ring4 * (1 << 50) - ring5 * (1 << 40)) / (1 << 33);

        println!("50");
        let day50: u64 = sc.read();
        let day50 = day50 - ring4 * (1 << 12) - ring5 * (1 << 10) - ring6 * (1 << 8);
        let ring1 = day50 / (1 << 50);
        let ring2 = (day50 - ring1 * (1 << 50)) / (1 << 25);
        let ring3 = (day50 - ring1 * (1 << 50) - ring2 * (1 << 25)) / (1 << 16);

        println!(
            "{} {} {} {} {} {}",
            ring1, ring2, ring3, ring4, ring5, ring6
        );

        let response: i64 = sc.read();
        assert_eq!(response, 1);
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
