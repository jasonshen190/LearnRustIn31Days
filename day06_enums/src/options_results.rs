pub fn options_results() {
    println!("=== Option 和 Result 使用 ===");
    
    // Option 处理可能为空的值
    let numbers = vec![1, 2, 3, 4, 5];
    
    match find_number(&numbers, 3) {
        Some(index) => println!("找到数字 3 在索引 {}", index),
        None => println!("没有找到数字 3"),
    }
    
    match find_number(&numbers, 10) {
        Some(index) => println!("找到数字 10 在索引 {}", index),
        None => println!("没有找到数字 10"),
    }
    
    // Result 处理可能失败的操作
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 ÷ 2 = {}", result),
        Err(err) => println!("除法错误: {}", err),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 ÷ 0 = {}", result),
        Err(err) => println!("除法错误: {}", err),
    }
    
    // 链式调用和 ? 操作符的替代
    println!("\n=== 处理多个可能失败的操作 ===");
    
    match parse_and_double("42") {
        Ok(result) => println!("解析并翻倍结果: {}", result),
        Err(err) => println!("操作失败: {}", err),
    }
    
    match parse_and_double("abc") {
        Ok(result) => println!("解析并翻倍结果: {}", result),
        Err(err) => println!("操作失败: {}", err),
    }
    
    // Option 的常用方法
    println!("\n=== Option 的常用方法 ===");
    
    let some_value = Some(42);
    let none_value: Option<usize> = None;
    
    println!("some_value.is_some(): {}", some_value.is_some());
    println!("none_value.is_none(): {}", none_value.is_none());
    
    // unwrap_or 提供默认值
    println!("some_value.unwrap_or(0): {}", some_value.unwrap_or(0));
    println!("none_value.unwrap_or(0): {}", none_value.unwrap_or(0));
    
    // map 转换 Option 中的值
    let doubled = some_value.map(|x| x * 2);
    println!("some_value 翻倍: {:?}", doubled);
}

fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

fn parse_and_double(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num * 2),
        Err(_) => Err(format!("无法解析 '{}' 为数字", s)),
    }
}