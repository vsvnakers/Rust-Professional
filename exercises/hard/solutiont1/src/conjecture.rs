pub fn goldbach_conjecture() -> String {
    // 判断一个数是否为素数
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    // 判断一个奇合数是否可以写成一个素数和一个平方的两倍之和
    fn can_be_expressed(n: u32) -> bool {
        let mut k = 1;
        while 2 * k * k < n {
            let remainder = n - 2 * k * k;
            if is_prime(remainder) {
                return true;
            }
            k += 1;
        }
        false
    }

    let mut results = Vec::new();
    let mut current = 9; // 从最小的奇合数开始

    while results.len() < 2 {
        if !is_prime(current) && !can_be_expressed(current) {
            results.push(current);
        }
        current += 2; // 只检查奇数
    }

    format!("{},{}", results[0], results[1])
}