#[derive(Debug, Clone)]
enum UserMessage {
    Text(String),
    Heartbeat,
    Close { code: u16, reason: String },
    Binary(Vec<u8>),
    Join { room: String, username: String },
    Leave { room: String },
}

#[derive(Debug)]
enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

pub fn enums_msg() {
    println!("=== WebSocket 消息处理 ===");
    
    // 创建不同类型的消息
    let messages = vec![
        UserMessage::Text("Hello, WebSocket!".to_string()),
        UserMessage::Heartbeat,
        UserMessage::Close {
            code: 1000,
            reason: "Normal closure".to_string(),
        },
        UserMessage::Binary(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]), // "Hello" in bytes
        UserMessage::Join {
            room: "general".to_string(),
            username: "Alice".to_string(),
        },
        UserMessage::Leave { room: "general".to_string() },
        
    ];
    
    for message in messages {
        handle_message(message);
    }
    
    // 连接状态处理
    println!("\n=== 连接状态处理 ===");
    let statuses = vec![
        ConnectionStatus::Connecting,
        ConnectionStatus::Connected,
        ConnectionStatus::Error("Network timeout".to_string()),
        ConnectionStatus::Disconnected,
    ];
    
    for status in statuses {
        handle_connection_status(status);
    }
}

fn handle_message(msg: UserMessage) {
    match msg {
        UserMessage::Text(content) => {
            println!("📝 收到文本消息: {}", content);
            // 这里可以添加文本消息的处理逻辑
        }
        UserMessage::Heartbeat => {
            println!("💓 收到心跳包");
            // 发送心跳响应
        }
        UserMessage::Close { code, reason } => {
            println!("🔌 连接关闭 - 代码: {}, 原因: {}", code, reason);
            // 清理资源
        }
        UserMessage::Binary(data) => {
            println!("📦 收到二进制数据: {} 字节", data.len());
            // 处理二进制数据
            if let Ok(text) = String::from_utf8(data) {
                println!("    转换为文本: {}", text);
            }
        }
        UserMessage::Join { room, username } => {
            println!("🚪 用户 {} 加入房间 {}", username, room);
            // 将用户添加到房间
        }
        UserMessage::Leave { room } => {
            println!("🚪 用户离开房间 {}", room);
            // 从房间移除用户
        }
    }
}

fn handle_connection_status(status: ConnectionStatus) {
    match status {
        ConnectionStatus::Connecting => {
            println!("🔄 正在连接...");
        }
        ConnectionStatus::Connected => {
            println!("✅ 连接已建立");
        }
        ConnectionStatus::Disconnected => {
            println!("❌ 连接已断开");
        }
        ConnectionStatus::Error(err_msg) => {
            println!("⚠️ 连接错误: {}", err_msg);
            // 这里可以添加错误处理逻辑
        }
    }
}