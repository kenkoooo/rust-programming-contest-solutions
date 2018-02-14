fn main() {
    let s = read_line().trim().bytes().collect::<Vec<_>>();
    let yah = "yah".to_owned().trim().bytes().collect::<Vec<_>>();

    if s[0] == yah[0] && s[1] == yah[1] && s[2] == yah[2] && s[3] == s[4] {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug, {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}