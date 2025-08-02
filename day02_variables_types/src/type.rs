pub fn demo() {
    println!("=== Rust 基本数据类型 ===");
    
    // 整数类型
    let small_int: i8 = 127;           // 8位有符号整数
    let big_int: i64 = 9223372036854775807; // 64位有符号整数
    let unsigned: u32 = 4294967295;    // 32位无符号整数
    
    println!("小整数: {}, 大整数: {}, 无符号: {}", small_int, big_int, unsigned);
    
    // 浮点类型
    let float32: f32 = 3.14159;
    let float64: f64 = 2.718281828459045;
    
    println!("32位浮点: {:.3}, 64位浮点: {:.6}", float32, float64);
    
    // 布尔类型
    let is_rust_awesome: bool = true;
    let is_learning_hard: bool = false;
    
    println!("Rust很棒吗? {}, 学习困难吗? {}", is_rust_awesome, is_learning_hard);
    
    // 字符类型（注意单引号）
    let heart_emoji: char = '❤';
    let letter: char = 'R';
    
    println!("字符: {} {}", heart_emoji, letter);
    
    // 字符串类型
    let greeting: &str = "Hello, Rust!";      // 字符串切片
    let owned_string: String = String::from("我在学习 Rust"); // 拥有所有权的字符串
    
    println!("字符串切片: {}", greeting);
    println!("String 类型: {}", owned_string);
}