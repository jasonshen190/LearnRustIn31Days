#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_count: u32,
    active: bool,
}

// 实现块（impl block）
impl User {
    // 关联函数（类似静态方法）
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            login_count: 0,
            active: true,
        }
    }
    
    // 方法（需要 self 参数）
    fn login(&mut self) {
        self.login_count += 1;
        println!("用户 {} 登录，总登录次数: {}", self.username, self.login_count);
    }
    
    fn describe(&self) -> String {
        format!("用户: {}, 邮箱: {}, 登录次数: {}, 状态: {}",
                self.username, 
                self.email, 
                self.login_count,
                if self.active { "活跃" } else { "非活跃" }) // dynamic
    }
    
    fn deactivate(&mut self) {
        self.active = false;
        println!("用户 {} 已被停用", self.username);
    }
    
    fn is_frequent_user(&self) -> bool {
        self.login_count > 10
    }
    
    // 消费 self 的方法
    fn delete_account(self) -> String {
        format!("用户 {} 的账户已被删除", self.username) // ownership
    }
}

pub fn structs_funcs() {
    println!("=== 结构体方法练习 ===");
    
    // 使用关联函数创建实例
    let mut user = User::new(
        String::from("Alice"),
        String::from("alice@example.com")
    );
    
    println!("新用户: {}", user.describe());
    
    // 调用方法
    user.login();
    user.login();
    user.login();
    
    println!("用户描述: {}", user.describe());
    println!("是否为频繁用户: {}", user.is_frequent_user());
    
    // 多次登录后再检查
    for _ in 0..10 {
        user.login();
    }
    
    println!("多次登录后是否为频繁用户: {}", user.is_frequent_user());
    
    user.deactivate();
    println!("停用后的用户: {}", user.describe());
    
    // 删除账户（消费 self）
    let deletion_message = user.delete_account();
    println!("{}", deletion_message);
    // println!("{:?}", user); // 编译错误：user 已被移动
}