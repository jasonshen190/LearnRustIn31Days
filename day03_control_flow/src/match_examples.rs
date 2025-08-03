pub fn matches() {
    println!("=== match 模式匹配练习 ===");
    
    // 基本 match 用法
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 => println!("四"),
        5 => println!("五"),
        _ => println!("其他数字"), // _ 是通配符，匹配所有其他情况
    }
    
    // match 匹配范围
    let score = 85;
    match score {
        90..=100 => println!("优秀"),
        80..=89 => println!("良好"),
        70..=79 => println!("一般"),
        60..=69 => println!("及格"),
        _ => println!("不及格"),
    }
    
    // match 表达式（返回值）
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("成绩等级: {}", grade);
    
    // 匹配多个值
    let day = 3;
    match day {
        1 | 2 | 3 | 4 | 5 => println!("工作日"),
        6 | 7 => println!("周末"),
        _ => println!("无效的日期"),
    }
    
    // 匹配条件（守卫）
    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("小于5的数: {}", x),
        Some(x) => println!("大于等于5的数: {}", x),
        None => println!("没有数字"),
    }
}