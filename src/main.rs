fn main() {
    let input_vec = read_to_vec::<usize>();
    let n = input_vec[0];
    let a = input_vec[1];
    let b = input_vec[2];
    println!("{}", resolve(n, a, b));
}

fn read_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
}

fn resolve(n: usize, a: usize, b: usize) -> usize {
    let filtered = (1..n+1).filter(|&i| {
        let sum = sum_in_each_digit(i);
        sum >= a && sum <= b
    });
    filtered.fold(0, |sum, i| sum + i)
}

fn sum_in_each_digit(i: usize) -> usize {
    let mut current = i;
    let mut sum = 0;
    while current !=0 {
        sum += current % 10;
        current = current / 10;
    }
    sum
}


#[cfg(test)]
mod tests {
    use ::{sum_in_each_digit, resolve};

    #[test]
    fn test_sum_in_each_digit() {
        let sum = sum_in_each_digit(258);
        assert_eq!(2+5+8, sum);
    }

    #[test]
    fn test_resolve() {
        let result = resolve(10, 1, 2);
        assert_eq!(result, 13)
    }
}
