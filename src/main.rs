fn main() {
    let n = read::<usize>();
    let mut d_list: Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        d_list.push(read::<usize>());
    }
    d_list.sort();
    d_list.dedup();
    println!("{}", d_list.len());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
