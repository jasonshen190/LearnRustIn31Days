pub fn slices() {
    println!("=== 切片练习 ===");
    
    let s = String::from("hello world");
    
    // 字符串切片
    let hello = &s[0..5];   // 或 &s[..5]
    let world = &s[6..11];  // 或 &s[6..]
    let whole = &s[..];     // 整个字符串的切片
    
    println!("原字符串: {}", s);
    println!("hello: {}, world: {}", hello, world);
    println!("整个字符串: {}", whole);
    
    // 使用切片的函数
    let first_word = get_first_word(&s);
    println!("第一个单词: {}", first_word);
    
    // 字符串字面量就是切片
    let s_literal = "Hello, world!"; // 类型是 &str
    let first = get_first_word(s_literal);
    println!("字面量的第一个单词: {}", first);
    
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("数组切片: {:?}", slice);
    
    // 切片作为参数的优势
    let my_string = String::from("hello world");
    let word1 = get_first_word(&my_string[0..6]); // String 的切片
    let word2 = get_first_word(&my_string[..]);    // 整个 String 的切片
    let word3 = get_first_word("hello world");     // 字符串字面量
    
    println!("word1: {}, word2: {}, word3: {}", 
             get_first_word(&word1), get_first_word(&word2), word3);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        println!("{item}"); // ASCII table code - https://www.ascii-code.com/ 
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..] // 没有空格，返回整个字符串
}