/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use self::bitset::*;

fn main() {
    input!(n: usize, a: [usize; n]);
    let sum: usize = a.iter().sum();

    let mut set = BitSet::new(sum);
    set.set(0, true);
    for &a in a.iter() {
        let next = set.shift_left(a);
        set.or_assign(next);
    }

    for i in ((sum + 1) / 2).. {
        if set.get(i) {
            println!("{}", i);
            return;
        }
    }
}

pub mod bitset {
    use std::ops::{BitOrAssign, Shl};
    const ONE_VALUE_LENGTH: usize = 60;
    const MAXIMUM: u64 = (1 << ONE_VALUE_LENGTH) - 1;

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
            self.or_assign(rhs);
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
                self.data[data_index] |= 1 << bit_index;
            } else {
                let tmp = MAXIMUM ^ (1 << bit_index);
                self.data[data_index] &= tmp;
            }
        }

        pub fn get(&mut self, index: usize) -> bool {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(
                self.data.len() > data_index,
                "{} {}",
                self.data.len(),
                data_index
            );
            self.data[data_index] & (1 << bit_index) != 0
        }

        pub fn or_assign(&mut self, rhs: Self) {
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

        pub fn shift_left(&self, shift: usize) -> Self {
            let mut next_data = Vec::new();
            let prefix_empty_count = shift / ONE_VALUE_LENGTH;
            let shift_count = shift % ONE_VALUE_LENGTH;
            for _ in 0..prefix_empty_count {
                next_data.push(0);
            }

            let mut from_previous = 0;
            let room = ONE_VALUE_LENGTH - shift_count;
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
