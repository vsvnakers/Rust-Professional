/*
    Find Missing Number in Array
    Given an array containing `n-1` numbers in the range from `1` to `n`, find the missing number.
    The array is not sorted, and each number in the range appears exactly once except one.
    You need to solve this problem in O(n) time complexity and O(1) space complexity.
    Implement the function `find_missing_number(nums: Vec<i32>) -> i32`.
    The function should return the missing number.
    
    You are required to find an optimal solution with O(n) time complexity and O(1) space complexity.
    
    Hint: Use the sum of the first `n` numbers and subtract the sum of the array elements to find the missing number.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_missing_number(nums: Vec<i32>) -> i32 {
    // TODO: Implement the logic to find the missing number
    // 计算数组长度，即 n-1
    let n = nums.len() as i32 + 1;
    
    // 计算 1 到 n 的和
    let expected_sum = n * (n + 1) / 2;
    
    // 计算数组中所有元素的和
    let actual_sum = nums.iter().sum::<i32>();
    
    // 返回缺失的数字
    expected_sum - actual_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number_1() {
        let nums = vec![3, 7, 1, 2, 8, 4, 5];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 6);
    }

    #[test]
    fn test_missing_number_2() {
        let nums = vec![1, 2, 4, 5];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 3);
    }

    #[test]
    fn test_missing_number_3() {
        let nums = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 1);
    }

    #[test]
    fn test_missing_number_4() {
        let nums = vec![1, 2, 3, 5, 6];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 4);
    }
}
