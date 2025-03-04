pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_prime = 0;

    // 处理2和3的因子
    for &prime in &[2, 3] {
        while number % prime == 0 {
            max_prime = prime;
            number /= prime;
        }
    }

    // 生成素数表
    let limit = number.isqrt().min(1_000_000) as u64;
    let small_primes = sieve_of_eratosthenes(limit);

    // 检查小素数因子
    for &prime in &small_primes {
        if number == 1 {
            break;
        }
        while number % prime as u128 == 0 {
            max_prime = prime as u128;
            number /= prime as u128;
        }
    }

    // 确定大素数因子的起始检查点
    let mut factor = if number.isqrt() > *small_primes.last().unwrap() as u128 {
        (*small_primes.last().unwrap() as u128).pow(2)
    } else {
        *small_primes.last().unwrap() as u128
    };

    while factor * factor <= number {
        for offset in [0, 2, 4, 6].iter() {
            let candidate = factor + offset;
            if number % candidate == 0 {
                max_prime = candidate;
                number /= candidate;
                break;
            }
        }
        factor += 8;
    }

    // 处理最后的素数
    if number > 2 {
        max_prime = number;
    }
    max_prime
}

/// 用埃拉托斯特尼筛法生成素数表
///
/// # 参数
/// * `limit` - 生成素数的上限，类型为u64
///
/// # 返回值
/// 返回一个包含所有小于等于`limit`的素数的Vec<u64>
///
/// # 算法说明
/// 1. 初始化一个布尔数组，标记所有数为素数
/// 2. 从2开始，将每个素数的倍数标记为非素数
/// 3. 最后收集所有标记为素数的数
///
/// # 复杂度
/// 时间复杂度：O(n log log n)
/// 空间复杂度：O(n)
fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=limit.isqrt() {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(index, _)| index as u64)
        .collect()
}