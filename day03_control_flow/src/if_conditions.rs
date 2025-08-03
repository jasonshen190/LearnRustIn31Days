pub fn if_conditions() {
    println!("=== if 条件语句练习 ===");
    
    let number = 42;
    
    // 基本 if 语句
    if number > 0 {
        println!("{} 是正数", number);
    }
    
    // if-else 语句
    if number % 2 == 0 {
        println!("{} 是偶数", number);
    } else {
        println!("{} 是奇数", number);
    }
    
    // if-else if-else 链
    let temperature = 25;
    if temperature > 30 {
        println!("天气很热！");
    } else if temperature > 20 {
        println!("天气刚好");
    } else if temperature > 10 {
        println!("有点凉");
    } else {
        println!("很冷！");
    }
    
    // if 表达式（可以返回值）
    let condition = true;
    let result: &'static str = if condition {
        "条件为真"
    } else {
        "条件为假"
    };
    println!("结果: {}", result);
    
    // 更复杂的条件判断
    let age = 18;
    let has_license = true;
    
    if age >= 18 && has_license {
        println!("可以开车");
    } else if age >= 18 {
        println!("已成年但没有驾照");
    } else {
        println!("未成年，不能开车");
    }
}