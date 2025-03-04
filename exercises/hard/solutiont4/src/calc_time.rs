use std::collections::{HashMap, HashSet};

// 判断是否为闰年
fn is_leapyear(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// 将字符串转换为无符号整数
fn str_to_num(s: &str) -> u32 {
    let bytes = s.as_bytes();
    bytes.iter().fold(0, |acc, &byte| acc * 10 + (byte - b'0') as u32)
}

// 计算指定日期的星期几
fn date_to_week_day(year: u32, month: u32, day: u32) -> u32 {
    let c = (year / 100) as i32;
    let mut y = (year % 100) as i32;
    let m: i32;
    let d = day as i32;
    
    if month == 1 || month == 2 {
        m = month as i32 + 12;
        y -= 1;
    } else {
        m = month as i32;
    }
    
    let mut h = (y + y/4 + c/4 - 2*c + (13*(m + 1)/5) + d - 1) % 7;
    if h <= 0 {
        h += 7;
    }
    
    h as u32
}

// 获取月份天数表，根据是否闰年调整二月天数
fn get_month_days(year: u32) -> [u32; 13] {
    let mut month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month_days[2] = 29;
    }
    month_days
}

// 计算日期在年内的天数分布
fn count_days(year: u32, month: u32, day: u32) -> (u32, u32) {
    let month_days = get_month_days(year);
    
    // 计算已过天数
    let days_passed = (1..month).map(|m| month_days[m as usize]).sum::<u32>() + day;
    
    // 计算剩余天数
    let total_days = if is_leapyear(year) { 366 } else { 365 };
    let days_remaining = total_days - days_passed;
    
    (days_passed, days_remaining)
}

// 计算指定日期在一年中的周数（ISO 8601 标准）
fn date_to_week_count(year: u32, month: u32, day: u32) -> u32 {
    let mut week_day = date_to_week_day(year, 1, 1);
    if week_day == 7 {
        week_day = 0;
    }
    
    let (days, _) = count_days(year, month, day);
    
    let ans = if week_day > 4 {
        // 第一周属于上一年的情况
        if day <= 7 - week_day + 1 {
            date_to_week_count(year - 1, 12, 31)
        } else {
            (days + week_day - 3) / 7 + 1
        }
    } else {
        (days + week_day - 2) / 7 + 1
    };
    
    // 处理12月末可能属于下一年第一周的情况
    if month == 12 {
        let last_day_week_day = date_to_week_day(year, 12, 31);
        if last_day_week_day < 4 && 31 - day < last_day_week_day {
            1
        } else {
            ans
        }
    } else {
        ans
    }
}

// 计算两个日期之间的天数差
fn compute_diff_of_date(year0: u32, month0: u32, day0: u32, year1: u32, month1: u32, day1: u32) -> u32 {
    if year0 == year1 {
        // 同年日期：直接计算两个日期的年内天数差
        let (days_0, _) = count_days(year0, month0, day0);
        let (days_1, _) = count_days(year1, month1, day1);
        days_0.saturating_sub(days_1)
    } else {
        // 跨年日期：计算第一个日期到年底的天数 + 第二个日期在年初的天数
        let (days_0, _) = count_days(year0, month0, day0);
        let (_, days_1) = count_days(year1, month1, day1);
        days_0 + days_1
    }
}

// 获取下一天的日期
fn next_day(year: u32, month: u32, day: u32) -> (u32, u32, u32) {
    let month_days = get_month_days(year);
    
    if day < month_days[month as usize] {
        // 当月未结束
        (year, month, day + 1)
    } else if month < 12 {
        // 当月结束，年未结束
        (year, month + 1, 1)
    } else {
        // 年结束
        (year + 1, 1, 1)
    }
}

// 计算距离最近春节的天数
fn date_to_newyear(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    // 获取当年春节日期
    let (newyear_month, newyear_day) = match date2newyear.get(&year) {
        Some(&date) => date,
        None => panic!("春节日期数据缺失：{}", year),
    };
    
    // 确定目标春节年份
    let (target_year, target_month, target_day) = if month > newyear_month 
            || (month == newyear_month && day > newyear_day) {
        // 当前日期已过当年春节，计算到下一年春节的天数
        let next_year_date = match date2newyear.get(&(year + 1)) {
            Some(&date) => date,
            None => panic!("春节日期数据缺失：{}", year + 1),
        };
        (year + 1, next_year_date.0, next_year_date.1)
    } else {
        // 当前日期未过当年春节
        (year, newyear_month, newyear_day)
    };
    
    // 计算天数差
    compute_diff_of_date(target_year, target_month, target_day, year, month, day)
}

// 判断指定日期是否为节假日（含周末调整）
fn is_holiday(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> bool {
    let month_days = get_month_days(year);
    let mut holidays: HashSet<(u32, u32)> = HashSet::new();
    
    // 添加元旦
    holidays.insert((1, 1));
    
    // 添加春节及前后假期
    let (newyear_month, newyear_day) = match date2newyear.get(&year) {
        Some(&date) => date,
        None => panic!("春节日期数据缺失：{}", year),
    };
    
    // 添加除夕
    if newyear_day > 1 {
        holidays.insert((newyear_month, newyear_day - 1));
    } else if newyear_month > 1 {
        holidays.insert((newyear_month - 1, month_days[(newyear_month - 1) as usize]));
    } else {
        // 春节在1月1日的极端情况，除夕为前一年12月31日
        holidays.insert((12, 31));
    }
    
    // 添加春节七天假期
    let mut current_date = (year, newyear_month, newyear_day);
    for _ in 0..7 {
        holidays.insert((current_date.1, current_date.2));
        current_date = next_day(current_date.0, current_date.1, current_date.2);
    }
    
    // 添加劳动节五天假期
    for day in 1..=5 {
        holidays.insert((5, day));
    }
    
    // 判断是节假日或周末
    if holidays.contains(&(month, day)) {
        true
    } else {
        let week_day = date_to_week_day(year, month, day);
        week_day >= 6
    }
}

// 计算到下一个交易日的天数差
fn date_to_trading_days(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut trading_day = next_day(year, month, day);
    
    // 查找下一个非假日
    while is_holiday(trading_day.0, trading_day.1, trading_day.2, date2newyear) {
        trading_day = next_day(trading_day.0, trading_day.1, trading_day.2);
    }
    
    // 计算天数差并减去当天
    compute_diff_of_date(trading_day.0, trading_day.1, trading_day.2, year, month, day) - 1
}

// 处理时间信息并返回格式化结果
pub fn time_info(time: &str) -> String {
    // 解析日期
    let parts: Vec<&str> = time.split('-').collect();
    if parts.len() != 3 {
        return "格式错误: 请使用yyyy-MM-dd格式".to_string();
    }
    
    let year = str_to_num(parts[0]);
    let month = str_to_num(parts[1]);
    let day = str_to_num(parts[2]);
    
    // 计算各项信息
    let week_count = date_to_week_count(year, month, day);
    let week_day = date_to_week_day(year, month, day);
    let (days_passed, days_remaining) = count_days(year, month, day);
    
    // 初始化春节日期映射表
    let mut date2newyear = HashMap::new();
    date2newyear.insert(2025, (1, 29));
    date2newyear.insert(2026, (2, 17));
    // 这里应添加更多年份的春节数据
    
    // 计算到春节的天数和到交易日的天数
    let days_to_newyear = date_to_newyear(year, month, day, &date2newyear);
    let days_to_trading = date_to_trading_days(year, month, day, &date2newyear);
    
    // 格式化输出
    format!(
        "{},{},{},{},{},{}",
        week_count, 
        week_day, 
        days_passed, 
        days_remaining, 
        days_to_newyear, 
        days_to_trading
    )
}