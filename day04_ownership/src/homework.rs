// 编写函数计算字符串中单词的数量（使用借用）
// 实现一个函数交换两个字符串的内容（使用可变引用）
// 创建一个函数返回字符串的最后一个单词
// 编写代码演示所有权规则的各种场景

pub fn homework() {

let s_1 = String::from("Hello, this is       Rust, World!!!!!");
let s_len: i32 = get_words_count(&s_1);

println!("There are {} words in the string", s_len);

let mut s_21 = String::from(",World");
let mut s_22 = String::from("Hello");

println!("交换前: s_21={}, s_22={}", s_21, s_22);
swap_strings(&mut s_21, &mut s_22);
println!("交换后: s_21={}, s_22={}", s_21, s_22);

let s_3 = String::from("Hello, this is       Rust, World!!!!!");
let s_3_last_word = get_last_word(&s_3);
println!("最后一个单词是: '{}'", s_3_last_word);
}


fn get_words_count(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut word_count = 0;
    let mut in_word = false;
    
    for (_i, &byte) in bytes.iter().enumerate() {
        // 检查是否为字母 (A-Z: 65-90, a-z: 97-122)
        let is_letter = (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122);
        
        if is_letter {
            if !in_word {
                word_count += 1;
                in_word = true;
            }
        } else {
            in_word = false;
        }
    }
    
    println!("There are {} words in the string", word_count);
    word_count
}

fn swap_strings(s1: &mut String, s2: &mut String) {
    // 手动实现字符串交换，不使用内置的swap
    // 1. 将s1的内容保存到临时变量
    let mut temp = String::new();
    
    // 2. 逐字符复制s1到temp
    for ch in s1.chars() {
        temp.push(ch);
    }
    
    // 3. 清空s1并复制s2的内容到s1
    s1.clear();
    for ch in s2.chars() {
        s1.push(ch);
    }
    
    // 4. 清空s2并复制temp的内容到s2
    s2.clear();
    for ch in temp.chars() {
        s2.push(ch);
    }
}

// // 更简单的字符串交换方式
// fn swap_strings_simple(s1: &mut String, s2: &mut String) {
//     let temp = s1.clone();  // clone()复制整个字符串内容
//     *s1 = s2.clone();       // 直接赋值，替换整个字符串
//     *s2 = temp;             // 赋值临时变量到s2
// }

fn get_last_word(s: &str) -> &str {
    let bytes = s.as_bytes();                    // 将字符串转换为字节数组以便索引访问
    let len = bytes.len();                       // 获取字节数组的长度
    
    if len == 0 {                                // 如果字符串为空
        return "";                               // 直接返回空字符串
    }
    
    // 从后往前找到最后一个字母字符的位置（单词结束位置）
    let mut word_end = None;                     // 用于存储最后一个字母的位置+1
    for i in (0..len).rev() {                    // 从字符串末尾开始逆向遍历
        let byte = bytes[i];                     // 获取当前位置的字节值
        // 检查是否为字母 (A-Z: 65-90, a-z: 97-122)
        if (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122) {  // 判断是否为ASCII字母
            word_end = Some(i + 1);              // 记录字母结束位置（i+1是切片的结束索引）
            break;                               // 找到第一个字母就退出循环
        }
    }
    
    // 如果没有找到字母字符
    let end = match word_end {                   // 模式匹配处理Option类型
        Some(pos) => pos,                        // 如果有值，取出位置
        None => return "",                       // 如果没有找到字母，返回空字符串
    };
    
    // 从找到的字母位置继续往前找到单词开始位置
    let mut word_start = 0;                      // 单词开始位置，默认从字符串开头
    for i in (0..end).rev() {                    // 从刚找到的字母位置往前遍历
        let byte = bytes[i];                     // 获取当前位置的字节值
        // 如果不是字母，说明单词从下一个位置开始
        if !((byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)) {  // 如果不是字母
            word_start = i + 1;                  // 单词从下一个位置开始
            break;                               // 找到非字母字符就退出循环
        }
    }
    
    // 返回最后一个单词的切片（只包含字母）
    &s[word_start..end]                          // 返回从word_start到end的字符串切片
}