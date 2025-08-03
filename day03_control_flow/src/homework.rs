pub fn homework() {
//编写一个猜数字游戏（使用 loop 和 match）
let numbers = 1..=100;
for i in numbers {
    println!("Is it number {}?", i);
    match i {
        47 => {
            println!("Bingo!");
            break;
        },
        _ => {println!("Continue...")},
    }

}
//实现一个简单的计算器，根据操作符执行不同运算
let a = 1;
let b = 3 ;
let operator = Some("/");
match operator {
    Some(x) if x == "+" => println!("{}", a + b),
    Some(x) if x == "-" => println!("{}", a - b),
    Some(x) if x == "*" => println!("{}", a * b),
    Some(x) if x == "/" => println!("{:.2}", a as f64 / b as f64),
    _ => println!("Invalid"),
}
// 创建一个九九乘法表

for i in 1..=9 {
    for j in 1..=9{
        println!("{} * {} = {}", i, j, i * j);
    }
}
// 编写一个函数判断年份是否为闰年

let year_list = [1920, 1924, 1932, 1950, 1928, 1800, 1600];
let year_str_list = ["1920", "1924", "1932", "1950", "1928", "1800", "1600"];
for year in year_list {
    if year % 4 == 0 && ( year % 100 != 0 || year % 400 == 0 ){
        println!("{} 是闰年!", year);
    } else {
        println!("{} 不是闰年!", year);
    }
}
for year in year_str_list {
    let year: i32 = year.parse().expect("fail");
    if year % 4 == 0 && ( year % 100 != 0 || year % 400 == 0 ){
        println!("{} 是闰年!", year);
    } else {
        println!("{} 不是闰年!", year);
    }
}

}