pub fn loops() {
    println!("=== 循环语句练习 ===");
    
    // 1. loop 无限循环（需要显式 break）
    println!("--- loop 循环 ---");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("循环第 {} 次", counter);
        
        if counter == 3 {
            break; // 跳出循环
        }
    }
    
    // loop 可以返回值
    let mut multiply = 1;
    let result = loop {
        multiply *= 2;
        if multiply > 10 {
            break multiply; // 返回 multiply 的值
        }
    };
    println!("loop 返回值: {}", result);
    
    // 2. while 条件循环
    println!("--- while 循环 ---");
    let mut number = 3;
    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }
    println!("发射！");
    
    // 3. for 循环
    println!("--- for 循环 ---");
    
    // 遍历数组
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers {
        println!("数字: {}", num);
    }
    
    // 使用范围
    for i in 0..=5 {  // 0 到 5（包含5）
        println!("范围循环: {}", i);
    }
    
    // 倒序循环 
    for i in (1..4).rev() { // 3 到 1
        println!("倒序: {}", i);
    }
    
    // 带索引的循环
    let fruits = ["苹果", "香蕉", "橙子"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("第{}个水果: {}", index + 1, fruit);
    }

   // ownership

   let char_example_1: &'static str = "abc";
   for str in char_example_1.chars() {
    println!("String {}", str);
   }

   let char_example_2: [&'static str; 1] = ["abc"];
   for str in char_example_2.iter() {
    println!("String {}", str);
   }

}