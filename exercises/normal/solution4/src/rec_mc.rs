pub fn dp_rec_mc(amount: u32) -> u32 {
    // 定义硬币面值
    let denominations = [1, 2, 5, 10, 20, 50, 100];
    // 创建一个向量，用于存储每个金额所需的最少硬币数量，初始值为 amount + 1
    let mut min_coins = vec![amount + 1; (amount + 1) as usize];
    // 金额为 0 时所需硬币数量为 0
    min_coins[0] = 0;

    // 遍历每个金额从 1 到 amount
    for current_amount in 1..=amount {
        // 遍历每个硬币面值
        for &coin in &denominations {
            // 如果硬币面值不大于当前金额
            if coin <= current_amount {
                // 更新当前金额所需的最少硬币数量
                let remaining_amount = (current_amount - coin) as usize;
                min_coins[current_amount as usize] = min_coins[current_amount as usize].min(min_coins[remaining_amount] + 1);
            }
        }
    }

    // 返回凑出金额 amount 所需的最少硬币数量
    min_coins[amount as usize]
}
