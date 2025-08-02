// 创建一个函数，计算两个数的平均值
// 编写一个函数，判断一个数是否为偶数
// 实现一个简单的BMI计算器函数
// 尝试不同的数值类型转换


fn calc_avg(a: i32, b: i32) -> f64 {
    return (a + b) as f64 / 2.0;
}

fn is_even(a: u8) -> bool {
    if a % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn calc_bmi(weight: f64, height: f64) {
    let bmi_value = weight / (height * height);

    if bmi_value < 18.5 {
        println!("BMI category: 偏瘦");
    } else if bmi_value < 24.9 {
        println!("BMI category: 正常");
    } else if bmi_value < 29.9 {
        println!("BMI category: 超重");
    } else {
        println!("BMI category: 肥胖");
    }
}

fn value_convert() {
     // 类型转换示例
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    let half_float_val: f64 = float_val / 2.0;
    let back_to_int: i32 = float_val as i32;
    
    println!("整数: {:.2} -> 浮点: {:.2} -> 整数: {:.2}", int_val, float_val, back_to_int);
    println!("float value calculation: {:.2}", half_float_val);
}

pub fn homework() {
    let avg = calc_avg(10, 20);
    println!("(10 + 20) / 2 = {}", avg);

    let num_list: Vec<u8> = vec![1,2,3,4,6,12,31,7];
    for num in num_list {
        let is_even_result: bool = is_even(num);
        println!("Is {} a even number? Result: {}", num, is_even_result);
    }

    calc_bmi(102.5, 1.75);

    value_convert();
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        homework();
    }
}
// fn value_convert()