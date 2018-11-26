const INF: usize = 1e15 as usize;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let p: Vec<usize> = sc.read_vec(n);

    let mut graph = vec![vec![]; n];
    let mut inverted = vec![0; n];
    for i in 0..n {
        let from = p[i] - 1;
        let to = i;
        graph[from].push(to);
        inverted[to] = from;
    }

    let cycled_node = {
        let mut visited = vec![false; n];
        let mut cur = 0;
        while !visited[inverted[cur]] {
            cur = inverted[cur];
            visited[cur] = true;
        }
        cur
    };

    let mut cur = cycled_node;
    let mut cycle = vec![cur];
    cur = inverted[cur];
    while cur != cycle[0] {
        cycle.push(cur);
        cur = inverted[cur];
    }

    let mut is_cycle = vec![false; n];
    for &node in cycle.iter() {
        is_cycle[node] = true;
    }

    let mut num = vec![INF; n];
    for &node in cycle.iter() {
        dfs(node, &graph, &mut num, &is_cycle);
    }

    let head = cycled_node;
    let candidate1 = num[head];
    let mut next_nums: Vec<usize> = graph[head].iter().map(|&next| num[next]).collect();
    next_nums.push(candidate1);
    next_nums.sort();
    let mut candidate2 = 0;
    for &num in next_nums.iter() {
        if candidate2 == num {
            candidate2 += 1;
        }
    }

    let mut num1 = num.clone();
    num1[head] = candidate1;
    let mut cur = inverted[head];
    while cur != head {
        num1[cur] = get_grundy(cur, &graph, &num1);
        cur = inverted[cur];
    }
    if num1[head] == get_grundy(head, &graph, &num1) {
        println!("POSSIBLE");
        return;
    }

    let mut num2 = num.clone();
    num2[head] = candidate2;
    let mut cur = inverted[head];
    while cur != head {
        num2[cur] = get_grundy(cur, &graph, &num2);
        cur = inverted[cur];
    }
    if num2[head] == get_grundy(head, &graph, &num2) {
        println!("POSSIBLE");
        return;
    }

    println!("IMPOSSIBLE");
}

fn get_grundy(node: usize, graph: &Vec<Vec<usize>>, num: &Vec<usize>) -> usize {
    let mut next_nums: Vec<usize> = graph[node].iter().map(|&next| num[next]).collect();
    next_nums.sort();
    let mut grundy = 0;
    for &num in next_nums.iter() {
        if grundy == num {
            grundy += 1;
        }
    }
    grundy
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, num: &mut Vec<usize>, is_cycle: &Vec<bool>) -> usize {
    let mut next_nums: Vec<_> = graph[v]
        .iter()
        .filter(|&&next| !is_cycle[next])
        .map(|&next| dfs(next, graph, num, is_cycle))
        .collect();
    next_nums.sort();

    let mut cur = 0;
    for &next_num in next_nums.iter() {
        if cur == next_num {
            cur += 1;
        }
    }
    num[v] = cur;
    num[v]
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
