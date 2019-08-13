
fn count_pattern(a: u16, b: u16, c: u16, n: u16) -> u64 {
    let mut counter: u64 = 0;
    for i_a in 0..a+1 {
        for i_b in 0..b+1 {
            for i_c in 0..c+1 {
                let sum = i_a * 500 + i_b * 100 + i_c * 50;
                if sum == n {
                    counter += 1;
                }
            }
        }
    }
    counter
}

