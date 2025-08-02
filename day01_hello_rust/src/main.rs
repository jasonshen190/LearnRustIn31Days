fn main() {
    println!("=== Day 1: Rust Environment Setup ===");
    
    // 基本计算器
    let a = 10;
    let b = 3;
    
    println!("数字 {} 和 {} 的运算结果：", a, b);
    println!("加法: {} + {} = {}", a, b, add(a, b));
    println!("减法: {} - {} = {}", a, b, subtract(a, b));
    println!("乘法: {} × {} = {}", a, b, multiply(a, b));
    println!("除法: {} ÷ {} = {:.2}", a, b, divide(a, b));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: i32, y: i32) -> f64 {
    x as f64 / y as f64
}