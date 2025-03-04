pub fn new_birthday_probability(n: u32) -> f64 {
    // 如果人数小于等于1，概率为0
    if n <= 1 {
        return 0.0;
    }
    // 如果人数大于等于365，概率为1
    if n >= 365 {
        return 1.0;
    }

    // 存储生日的不同概率
    let mut probability = 1.0;

    // 计算概率
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }

    // 返回有两个人在同一天过生日的概率
    1.0 - probability
}
