pub fn nest_loop() {
    println!("=== 嵌套循环和标签 ===");
    
    // 嵌套循环
    for i in 1..=3 {
        for j in 1..=3 {
            println!("i={}, j={}", i, j);
        }
    }
    
    // 使用标签控制外层循环
    // 'outer is a loop label; break 'outer exits the outer for-loop directly.
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("在 i={}, j={} 时跳出外层循环", i, j);
                break 'outer; // 跳出外层循环
            }
            println!("i={}, j={}", i, j);
        }
    }
    
    // continue 跳过本次循环
    println!("跳过偶数:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("奇数: {}", i);
    }

    println!("=== continue 'outer 示例 ===");
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if j == 2 {
                println!("continue 'outer 触发: i={}, j={}", i, j);
                continue 'outer; // 跳过当前外层 i 的这次循环
            }
            println!("i={}, j={}", i, j);
        }
    }
}