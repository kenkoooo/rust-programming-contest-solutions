struct Scanner {
    buffer: std::collections::VecDeque<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { buffer: std::collections::VecDeque::new() }
    }

    fn read_line() -> String {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf
    }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        if self.buffer.is_empty() {
            for s in Scanner::read_line().split(' ') {
                self.buffer.push_back(s.to_string());
            }
        }
        self.buffer.pop_front().unwrap().trim().parse().unwrap()
    }
}
