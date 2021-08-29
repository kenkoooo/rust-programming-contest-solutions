use crate::mod_int::ModInt;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let x: i64 = sc.read();

    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for _ in 0..m {
        let u = sc.usize0();
        let v = sc.usize0();
        let w: i64 = sc.read();
        graph[u].push((v, w));
        graph[v].push((u, w));
        edges.push((w, u, v));
    }

    edges.sort();
    let mut uf = UnionFind::new(n);
    let mut mst_cost = 0;
    let mut mst = vec![vec![]; n];
    let mut non_mst = vec![];
    for (w, u, v) in edges {
        if uf.find(u) != uf.find(v) {
            mst_cost += w;
            uf.unite(u, v);
            mst[u].push((v, w));
            mst[v].push((u, w));
        } else {
            non_mst.push((w, u, v));
        }
    }

    let mut path_max = vec![vec![0; n]; n];
    for i in 0..n {
        dfs(i, i, &mst, &mut path_max[i]);
    }

    let required = x - mst_cost;
    if required < 0 {
        println!("0");
        return;
    }

    let mut equal = 0;
    let mut upper = 0;
    for (w, v, u) in non_mst {
        let diff = w - path_max[v][u];
        if diff == required {
            equal += 1;
        } else if diff > required {
            upper += 1;
        }
    }

    if required > 0 {
        let ans = ModInt::from(2) * (ModInt::from(2).pow(equal) - 1) * ModInt::from(2).pow(upper);
        println!("{}", ans.value());
        return;
    }

    let ans1 = (ModInt::from(2).pow(n - 1) - 2) * ModInt::from(2).pow(m - n + 1);
    let ans2 = (ModInt::from(2).pow(equal) - 1) * 2 * ModInt::from(2).pow(upper);
    println!("{}", (ans1 + ans2).value());
}

fn dfs(v: usize, p: usize, mst: &Vec<Vec<(usize, i64)>>, path_max: &mut Vec<i64>) {
    for &(next, e) in mst[v].iter() {
        if next == p {
            continue;
        }
        path_max[next] = path_max[v].max(e);
        dfs(next, v, mst, path_max);
    }
}

pub mod mod_int {
    type ModInternalNum = i64;
    thread_local!(
        static MOD: std::cell::RefCell<ModInternalNum> = std::cell::RefCell::new(0);
    );

    pub fn set_mod_int<T: ToInternalNum>(v: T) {
        MOD.with(|x| x.replace(v.to_internal_num()));
    }
    fn modulo() -> ModInternalNum {
        1_000_000_007
    }

    pub struct ModInt(ModInternalNum);
    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}

    impl ModInt {
        fn internal_new(mut v: ModInternalNum) -> Self {
            let m = modulo();
            if v >= m {
                v %= m;
            }
            Self(v)
        }

        pub fn internal_pow(&self, mut e: ModInternalNum) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let modulo = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= modulo;
                }
                e >>= 1;
                cur = (cur * cur) % modulo;
            }
            Self(result)
        }

        pub fn pow<T>(&self, e: T) -> Self
        where
            T: ToInternalNum,
        {
            self.internal_pow(e.to_internal_num())
        }

        pub fn value(&self) -> ModInternalNum {
            self.0
        }
    }

    pub trait ToInternalNum {
        fn to_internal_num(&self) -> ModInternalNum;
    }
    impl ToInternalNum for ModInt {
        fn to_internal_num(&self) -> ModInternalNum {
            self.0
        }
    }
    macro_rules! impl_primitive {
        ($primitive:ident) => {
            impl From<$primitive> for ModInt {
                fn from(v: $primitive) -> Self {
                    let v = v as ModInternalNum;
                    Self::internal_new(v)
                }
            }
            impl ToInternalNum for $primitive {
                fn to_internal_num(&self) -> ModInternalNum {
                    *self as ModInternalNum
                }
            }
        };
    }
    impl_primitive!(u8);
    impl_primitive!(u16);
    impl_primitive!(u32);
    impl_primitive!(u64);
    impl_primitive!(usize);
    impl_primitive!(i8);
    impl_primitive!(i16);
    impl_primitive!(i32);
    impl_primitive!(i64);
    impl_primitive!(isize);

    impl<T: ToInternalNum> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }

            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Add<T> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    impl<T: ToInternalNum> std::ops::Sub<T> for ModInt {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
    impl<T: ToInternalNum> std::ops::Mul<T> for ModInt {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = Self(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T: ToInternalNum> std::ops::Div<T> for ModInt {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        true
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
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
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
