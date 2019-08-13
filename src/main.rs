fn main() {
    let _n = read::<usize>();
    let a_list = read_to_vec::<usize>();
    println!("{}", resolve(a_list));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
}

fn resolve(mut a_list: Vec<usize>) -> usize {
    a_list.sort();
    a_list.reverse();
    let mut alice = 0;
    let mut bob = 0;
    for (i, &a) in a_list.iter().enumerate() {
        if i % 2 == 0 {
            alice += a;
        } else {
            bob += a;
        }
    }
    alice - bob
}


#[cfg(test)]
mod tests {
    use ::{sum_in_each_digit, resolve};

    #[test]
    fn test_resolve() {
        let result = resolve(vec![20, 18 , 2, 18]);
        assert_eq!(result, 18);
    }
}
