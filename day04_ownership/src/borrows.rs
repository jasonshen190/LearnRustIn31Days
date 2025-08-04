pub fn borrows() {
    println!("=== 引用和借用 ===");
    
    let s1 = String::from("hello");
    
    // 不可变引用
    let len = calculate_length(&s1); // 借用 s1
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然有效
    
    // 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后: {}", s2);
    
    // 引用规则演示
    let mut s3 = String::from("hello");
    
    // 可以有多个不可变引用
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 和 r2 作用域结束
    
    // 然后可以有一个可变引用
    let r3 = &mut s3;
    r3.push_str(", world!");
    println!("r3: {}", r3);
    // 不能同时有可变和不可变引用
    
    // 悬垂引用预防
    // let reference_to_nothing = dangle(); // 这会编译错误
    let valid_string = no_dangle();
    println!("有效的字符串: {}", valid_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它只是引用，不会释放内存

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 这个函数会产生悬垂引用，编译器会阻止
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回对即将被释放内存的引用
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 直接返回所有权
}