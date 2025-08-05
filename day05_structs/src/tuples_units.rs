pub fn tuples_units() {
    println!("=== 特殊类型的结构体 ===");
    
    // 元组结构体 tuple
    #[derive(Debug)]
    struct Color(i32, i32, i32); // RGB
    
    #[derive(Debug)]
    struct Point(i32, i32, i32); // 3D坐标
    
    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);
    
    println!("黑色: {:?}", black);
    println!("原点: {:?}", origin);
    println!("黑色的红色分量: {}", black.0);
      println!("黑色的绿色分量: {}", black.1);
        println!("黑色的蓝色分量: {}", black.1);

    
    // 单元结构体（没有字段）
        #[derive(Debug)]
        struct AlwaysEqual;
        
        let subject = AlwaysEqual;
        println!("单元结构体: {:?}", subject);
    
    // 使用结构体的实际例子
    let rectangle = Rectangle { width: 30, height: 50 };
    println!("矩形: {:?}", rectangle);
    println!("矩形面积: {}", rectangle.area());
    println!("矩形周长: {}", rectangle.perimeter());
    
    let square = Rectangle::square(25);
    println!("正方形: {:?}, 面积: {}", square, square.area());
    
    // 比较矩形
    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 20, height: 10 };
    
    println!("rect1 能容纳 rect2 吗? {}", rect1.can_hold(rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
