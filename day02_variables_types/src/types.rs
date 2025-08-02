pub fn types() {
    println!("=== Rust 基本数据类型 ===");
    
    // Rust 中的整数类型支持从 i8、i16、i32、i64 到 i128（有符号），以及 u8 到 u128（无符号）。
    // 不支持 i2 或 i256 这样的类型。
    let small_int: i8 = 127;           // 8位有符号整数，范围：-128 到 127
    // 以 u 开头的类型（如 u8、u16、u32、u64、u128）是无符号整数类型，只能表示正整数和零，表示范围从 0 开始。
    let big_int: i64 = 9223372036854775807; // 64位有符号整数，范围：-9,223,372,036,854,775,808 到 9,223,372,036,854,775,807
    let unsigned: u32 = 4294967295;    // 32位无符号整数，范围：0 到 4,294,967,295
    
    println!("小整数: {}, 大整数: {}, 无符号: {}", small_int, big_int, unsigned);
    
    // 浮点类型
    let float32: f32 = 3.14159;                      // 32位单精度浮点数，精度约为6~7位十进制数字
    let float64: f64 = 2.718281828459045;            // 64位双精度浮点数，精度约为15~16位十进制数字
    
    println!("32位浮点: {:.3}, 64位浮点: {:.6}", float32, float64);
    
    // 布尔类型
    let is_rust_awesome: bool = true;                // 布尔值，仅能为 true 或 false
    let is_learning_hard: bool = false;              // 表示学习是否困难
    
    println!("Rust很棒吗? {}, 学习困难吗? {}", is_rust_awesome, is_learning_hard);
    
    // 字符类型（注意单引号）
    let heart_emoji: char = '❤';                     // 字符类型，支持 Unicode 字符（4字节），可存储 emoji 等
    let letter: char = 'R';                          // 一个字符，例如字母、标点、符号等
    
    println!("字符: {} {}", heart_emoji, letter);
    
    // 字符串类型
    let greeting: &str = "Hello, Rust!";      // 字符串切片
    let owned_string: String = String::from("我在学习 Rust"); // 拥有所有权的字符串
    
    println!("字符串切片: {}", greeting);
    println!("String 类型: {}", owned_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_run() {
        types();
    }
}