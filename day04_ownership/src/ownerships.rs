pub fn ownerships() {
    println!("=== 所有权基本概念 ===");
    
    // Rust 有一个栈和堆的概念， 
    // 1. 栈上的数据（Copy 类型）
    let x = 5;
    let y = x; // 复制，x 仍然有效
    println!("x: {}, y: {}", x, y); // 都可以使用
    
    // 2. 堆上的数据（非 Copy 类型）
    let s1 = String::from("hello");
    let s2 = s1; // 移动（move），s1 不再有效
    // println!("{}", s1); // 这行会编译错误！
    println!("s2: {}", s2);
    
    // 3. 克隆数据
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝
    println!("s3: {}, s4: {}", s3, s4); // 都可以使用
    
    // 4. 函数调用中的所有权
    let msg = String::from("Hello, Rust!");
    take_ownership(msg); // msg 的所有权被移动
    // println!("{}", msg); // 这行会编译错误！

    let msg = String::from("Hello Rust!");
    borrow_ownership(&msg);
    println!("msg 依然有效{}", msg); // 这行不应该会报错
    
    let num = 42;
    makes_copy(num); // 数字类型会被复制
    println!("num 仍然有效: {}", num); // 仍然可以使用
}

fn take_ownership(some_string: String) {
    println!("接收到字符串: {}", some_string);
} // some_string 离开作用域，内存被释放

fn makes_copy(some_integer: i32) {
    println!("接收到数字: {}", some_integer);
} // some_integer 离开作用域，但因为是 Copy 类型，没有特殊处理

fn borrow_ownership(some_msg: &str) {
    println!("borrow {}", some_msg);
}