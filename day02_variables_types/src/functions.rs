pub fn functions() {
    println!("=== 函数练习 ===");
    
    // 调用无参数函数
    say_hello();
    
    // 调用带参数函数
    greet_user("张三");
    greet_user_with_age("李四", 25);
    
    // 调用有返回值的函数
    let sum = add_numbers(10, 20);
    println!("10 + 20 = {}", sum);
    
    let area = calculate_circle_area(5.0);
    println!("半径为5的圆面积: {:.2}", area);
    
    // 使用表达式返回值的函数
    let max = find_maximum(15, 8);
    println!("15 和 8 的最大值: {}", max);
}

// 无参数，无返回值
fn say_hello() {
    println!("你好，世界！");
}

// 带参数，无返回值
fn greet_user(name: &str) {
    println!("欢迎你，{}！", name);
}

fn greet_user_with_age(name: &str, age: u32) {
    println!("你好 {}，你今年 {} 岁了", name, age);
}

// 带参数，有返回值
fn add_numbers(a: i32, b: i32) -> i32 {
    return a + b; // 显式返回
}

fn calculate_circle_area(radius: f64) -> f64 {
    3.14159 * radius * radius // 表达式返回（无分号）
}

// 更复杂的函数示例
fn find_maximum(a: i32, b: i32) -> i32 {
    if a > b {
        a // 返回 a
    } else {
        b // 返回 b
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        functions();
    }
}