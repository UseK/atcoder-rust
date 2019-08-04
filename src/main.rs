fn main() {
    let a = read::<usize>();
    let bc = read_vec::<usize>();
    let s = read::<String>();
    let sum = a + bc[0] + bc[1];
    println!("{} {}", sum, s);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}


fn harmony_inner(a: isize, b: isize) -> Option<isize> {
    let sum = a + b;
    if sum % 2 == 0 {
        Some(sum / 2)
    } else {
        None
    }
}

fn harmony(a: isize, b: isize) -> String {
    match harmony_inner(a, b) {
        Some(i) => i.to_string(),
        None => "IMPOSSIBLE".to_string()
    }
}

#[cfg(test)]
mod tests {
    use ::{harmony};

    #[test]
    fn test_harmony() {
        let result = harmony(2, 16);
        assert_eq!(result, "9");
    }

    #[test]
    fn test_str_harmony() {
        let result = harmony(0, 3);
        assert_eq!(result, "IMPOSSIBLE");
    }
}
