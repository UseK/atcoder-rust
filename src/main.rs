fn main() {
    let _m =  read::<String>();
    let input =  read::<String>();
    let numbers: Vec<u64> = input.split_whitespace().map(|s| s.parse::<u64>().ok().unwrap()).collect();
    println!("{}", count_shift(&numbers));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn count_shift(numbers: &Vec<u64>) -> usize {
    let mut cloned = numbers.clone();
    let mut counter = 0;
    while is_all_even(&cloned) {
        counter += 1;
        cloned = cloned.iter().map(|x| x / 2).collect();
    }
    counter
}

fn is_all_even(numbers: &Vec<u64>) -> bool {
    numbers.iter().all(|&x| is_even(x))
}

fn is_even(x: u64) -> bool {
    x % 2 == 0
}

#[cfg(test)]
mod tests {
    use ::{is_all_even, count_shift};

    #[test]
    fn test_is_all_even() {
        let input = "382253568 723152896 37802240 379425024 404894720 471526144";
        let numbers: Vec<u64> = input.split_whitespace().map(|s| s.parse::<u64>().ok().unwrap()).collect();
        assert_eq!(true, is_all_even(&numbers));
        assert_eq!(true, is_all_even(&numbers));
    }

    #[test]
    fn test_shift_only() {
        let input = "382253568 723152896 37802240 379425024 404894720 471526144";
        let mut numbers: Vec<u64> = input.split_whitespace().map(|s| s.parse::<u64>().ok().unwrap()).collect();
        let mut counter = 0;
        while is_all_even(&numbers) {
            counter += 1;
            numbers = numbers.iter().map(|x| x / 2).collect();
        }
        assert_eq!(8, counter)
    }

    #[test]
    fn test_count_shift() {
        let input = "382253568 723152896 37802240 379425024 404894720 471526144";
        let numbers: Vec<u64> = input.split_whitespace().map(|s| s.parse::<u64>().ok().unwrap()).collect();
        assert_eq!(8, count_shift(&numbers));
    }
}


