pub fn type_inference_explicit_annotation() {
    // Rust 的类型推断很强大
    // help: if this is intentional, prefix it with an underscore: `_auto_int`
    let _auto_int = 42;          // 自动推断为 i32
    let _auto_float = 3.14;      // 自动推断为 f64
    let _auto_string = "hello";  // 自动推断为 &str
    
    // 有时需要显式标注
    let parsed_number: i32 = "412".parse().expect("不是有效数字");
    let bytes: Vec<u8> = vec![1, 2, 3, 4, 5];
    
    println!("解析的数字: {}", parsed_number);
    println!("字节数组: {:?}", bytes);
    
    // 类型别名
    type UserId = u64;
    type UserName = String;
    
    let user_id: UserId = 12345;
    let user_name: UserName = String::from("Rust学习者");
    
    println!("用户ID: {}, 用户名: {}", user_id, user_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        type_inference_explicit_annotation()
    }
}