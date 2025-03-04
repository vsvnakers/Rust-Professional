pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 将输入字符串按 '(' 分割，获取数字和源进制
    let parts: Vec<&str> = num_str.split('(').collect();
    let number = parts[0];
    let base_part = parts[1].split(')').collect::<Vec<&str>>()[0];
    
    // 将源进制转换为 u32
    let src_base = base_part.parse::<u32>().unwrap();
    
    // 将数字从源进制转换为 u32
    let num = u32::from_str_radix(number, src_base).expect("不是有效数字");
    
    // 如果数字为 0，直接返回 "0"
    if num == 0 {
        return "0".to_string();
    }
    
    // 存储结果
    let mut result = String::new();
    let mut n = num;
    
    // 将数字转换为目标进制
    while n > 0 {
        let digit = (n % to_base) as u32;
        let char = if digit < 10 {
            (digit as u8 + b'0') as char
        } else {
            (digit as u8 - 10 + b'a') as char
        };
        result.insert(0, char);
        n /= to_base;
    }
    
    result
}
