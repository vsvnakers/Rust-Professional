use::std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 将输入字符串按逗号分割，并收集到一个向量中
    let input_str: Vec<&str> = input_str.split(',').collect();

    // 创建一个 HashSet 来存储唯一的元素
    let mut cnt: HashSet<&str> = HashSet::new();
    
    // 遍历向量中的每个元素，并将其插入到 HashSet 中
    for i in 0..input_str.len() {
        cnt.insert(input_str[i]);
    }

    // 返回 HashSet 的长度，即唯一元素的数量
    cnt.len()
}
