pub fn demo() {
    // 不可变变量（默认）
    let x = 5;
    println!("x 的值是: {}", x);
    
    // x = 6; // 这行会编译错误！
    
    // 可变变量
    let mut y = 10;
    println!("y 的初始值: {}", y);
    y = 15; // 可以修改
    println!("y 的新值: {}", y);
    
    // 变量遮蔽（shadowing）
    let z = 5;
    let z = z + 1; // 创建新变量，遮蔽旧的
    println!("z current value: {}", z);
    
    // expect compile error
    // z = 1;

    // fix 
    let z = 1;
    println!("z new value is: {}", z);
    
    let z = z * 2;
    println!("z 的最终值: {}", z); // 输出 12
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_runs() {
        // 只验证 demo() 能正常执行（不 panic）
        demo();
    }
}

