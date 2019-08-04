fn main() {
    let s1s2s3 =  read::<String>();
    let counted = s1s2s3.chars().filter(|&x| x == '1').count();
    println!("{}", counted);
}

pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

