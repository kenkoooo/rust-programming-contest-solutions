use bitset::BitSet;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let mut ok = BitSet::new(2000 * 2000 + 1);
    ok.set(0, true);
    for i in 0..n {
        let mut next = ok.clone();
        next = next << a[i];
        ok |= next;
    }

    let sum = a.into_iter().sum::<usize>();

    for i in ((sum + 1) / 2).. {
        if ok.get(i) {
            println!("{}", i);
            return;
        }
    }
}
pub mod bitset {
    use std::ops::{BitOrAssign, Shl};
    const ONE_VALUE_LENGTH: usize = 63;
    const MAXIMUM: u64 = (1u64 << ONE_VALUE_LENGTH as u64) - 1;

    pub fn get_bit_position(index: usize) -> (usize, usize) {
        let data_index = index / ONE_VALUE_LENGTH;
        let bit_index = index % ONE_VALUE_LENGTH;
        (data_index, bit_index)
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BitSet {
        data: Vec<u64>,
    }

    impl BitOrAssign for BitSet {
        fn bitor_assign(&mut self, rhs: Self) {
            if self.data.len() < rhs.data.len() {
                self.data.resize(rhs.data.len(), 0);
            }
            let n = if self.data.len() > rhs.data.len() {
                rhs.data.len()
            } else {
                self.data.len()
            };
            for i in 0..n {
                assert!(self.data[i] <= MAXIMUM);
                assert!(rhs.data[i] <= MAXIMUM);
                self.data[i] |= rhs.data[i];
            }
        }
    }

    impl Shl<usize> for BitSet {
        type Output = Self;
        fn shl(self, rhs: usize) -> Self {
            self.shift_left(rhs)
        }
    }

    impl BitSet {
        pub fn new(n: usize) -> Self {
            let size = (n + ONE_VALUE_LENGTH - 1) / ONE_VALUE_LENGTH;
            BitSet {
                data: vec![0; size],
            }
        }

        pub fn new_from(value: u64) -> Self {
            BitSet { data: vec![value] }
        }

        pub fn set(&mut self, index: usize, value: bool) {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);
            if value {
                self.data[data_index] |= (1 << bit_index) as u64;
            } else {
                let tmp = MAXIMUM ^ (1 << bit_index) as u64;
                self.data[data_index] &= tmp;
            }
        }

        pub fn get(&mut self, index: usize) -> bool {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);
            self.data[data_index] & (1u64 << bit_index as u64) != 0
        }

        pub fn shift_left(&self, shift: usize) -> Self {
            let mut next_data = Vec::new();
            let prefix_empty_count = shift / ONE_VALUE_LENGTH;
            let shift_count = (shift % ONE_VALUE_LENGTH) as u64;
            for _ in 0..prefix_empty_count {
                next_data.push(0);
            }

            let mut from_previous = 0;
            let room = ONE_VALUE_LENGTH as u64 - shift_count;
            for &data in self.data.iter() {
                let overflow = (data >> room) << room;
                let rest = data - overflow;
                let value = (rest << shift_count) + from_previous;
                assert!(value <= MAXIMUM);
                next_data.push(value);
                from_previous = overflow >> room;
            }
            if from_previous > 0 {
                next_data.push(from_previous);
            }
            BitSet { data: next_data }
        }
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
