// 定义 User 结构体
#[derive(Debug)] // 自动实现 Debug trait，便于打印
struct User {
    username: String,
    email: String,
    login_count: u32,
    active: bool,
}

pub fn structs() {
    println!("=== 结构体基础使用 ===");
    
    // 创建结构体实例
    let user1 = User {
        username: String::from("张三"),
        email: String::from("zhangsan@example.com"),
        login_count: 0,
        active: true,
    };
    
    println!("用户信息: {:?}", user1);
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    println!("用户存在：{}", user1.active);
    
    // 可变结构体实例
    let mut user2 = User {
        username: String::from("李四"),
        email: String::from("lisi@example.com"),
        login_count: 5,
        active: true,
    };
    
    user2.login_count += 1; // 修改字段
    println!("李四登录次数更新为: {}", user2.login_count);
    
    // 使用函数创建结构体
    let user3 = build_user(
        String::from("王五"), 
        String::from("wangwu@example.com")
    );
    println!("通过函数创建的用户: {:?}", user3);
    
    // 结构体更新语法
    let user4 = User {
        username: String::from("赵六"),
        email: String::from("zhaoliu@example.com"),
        ..user1 // 使用 user1 的其他字段值
    };
    println!("使用更新语法创建的用户: {:?}", user4);

    let user5 = User {
        username: String::from("孙七"),
        ..user2
    };

    println!("User 5: {:#?}", user5);
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // 字段初始化简写
        email,    // 等同于 email: email
        login_count: 0,
        active: true,
    }
}