#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("退出应用程序");
            }
            Message::Move { x, y } => {
                println!("移动到坐标 ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("显示文本: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("改变颜色为 RGB({}, {}, {})", r, g, b);
            }
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
    
    fn get_description(&self) -> String {
        match self {
            Message::Quit => "退出消息".to_string(),
            Message::Move { .. } => "移动消息".to_string(),
            Message::Write(_) => "文本消息".to_string(),
            Message::ChangeColor(_, _, _) => "颜色变更消息".to_string(),
        }
    }
}

pub fn enums_func() {
    println!("=== 枚举方法演示 ===");
    
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello, Rust!".to_string()),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    
    for msg in &messages {
        println!("消息类型: {}", msg.get_description());
        msg.process();
        println!("是否为退出消息: {}", msg.is_quit());
        println!();
    }
}