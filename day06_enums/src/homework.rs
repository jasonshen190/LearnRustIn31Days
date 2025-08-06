// 创建一个 `OrderStatus` 枚举，表示订单的不同状态
// 实现一个 `HttpResponse` 枚举，处理不同的HTTP响应
// 设计一个游戏状态枚举，包含开始、游戏中、暂停、结束等状态
// 创建一个表示不同几何形状的枚举，并计算面积

use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
}

impl OrderStatus {
    pub fn can_cancel(&self) -> bool {
        matches!(self, OrderStatus::Pending | OrderStatus::Processing)
    }
    
    pub fn to_string(&self) -> &str {
        match self {
            OrderStatus::Pending => "待处理",
            OrderStatus::Processing => "处理中",
            OrderStatus::Shipped => "已发货",
            OrderStatus::Delivered => "已送达",
            OrderStatus::Cancelled => "已取消",
            OrderStatus::Returned => "已退回",
        }
    }
}

#[derive(Debug, Clone)]
pub enum HttpResponse {
    Ok(String),
    NotFound,
    InternalServerError(String),
    BadRequest(String),
    Unauthorized,
    Forbidden,
}

impl HttpResponse {
    pub fn status_code(&self) -> u16 {
        match self {
            HttpResponse::Ok(_) => 200,
            HttpResponse::BadRequest(_) => 400,
            HttpResponse::Unauthorized => 401,
            HttpResponse::Forbidden => 403,
            HttpResponse::NotFound => 404,
            HttpResponse::InternalServerError(_) => 500,
        }
    }
    
    pub fn message(&self) -> String {
        match self {
            HttpResponse::Ok(msg) => msg.clone(),
            HttpResponse::NotFound => "Not Found".to_string(),
            HttpResponse::InternalServerError(msg) => format!("Internal Server Error: {}", msg),
            HttpResponse::BadRequest(msg) => format!("Bad Request: {}", msg),
            HttpResponse::Unauthorized => "Unauthorized".to_string(),
            HttpResponse::Forbidden => "Forbidden".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Start,
    Playing,
    Paused,
    GameOver,
    Victory,
}

impl GameState {
    pub fn can_pause(&self) -> bool {
        matches!(self, GameState::Playing)
    }
    
    pub fn can_resume(&self) -> bool {
        matches!(self, GameState::Paused)
    }
    
    pub fn can_start(&self) -> bool {
        matches!(self, GameState::Start | GameState::GameOver | GameState::Victory)
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
    Square { side: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
            Shape::Square { side } => side * side,
        }
    }
    
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                let side = (height * height + (base / 2.0) * (base / 2.0)).sqrt();
                base + 2.0 * side
            }
            Shape::Square { side } => 4.0 * side,
        }
    }
}

pub fn homework() {
    println!("=== 枚举作业演示 ===");
    
    // OrderStatus 演示
    println!("\n1. 订单状态枚举演示:");
    let order = OrderStatus::Pending;
    println!("订单状态: {}", order.to_string());
    println!("可以取消: {}", order.can_cancel());
    
    let delivered_order = OrderStatus::Delivered;
    println!("已送达订单可以取消: {}", delivered_order.can_cancel());
    
    // HttpResponse 演示
    println!("\n2. HTTP响应枚举演示:");
    let responses = vec![
        HttpResponse::Ok("操作成功".to_string()),
        HttpResponse::NotFound,
        HttpResponse::InternalServerError("数据库连接失败".to_string()),
        HttpResponse::BadRequest("参数错误".to_string()),
    ];
    
    for response in responses {
        println!("状态码: {}, 消息: {}", response.status_code(), response.message());
    }
    
    // GameState 演示
    println!("\n3. 游戏状态枚举演示:");
    let mut game_state = GameState::Start;
    println!("当前游戏状态: {:?}", game_state);
    println!("可以开始: {}", game_state.can_start());
    
    game_state = GameState::Playing;
    println!("游戏中，可以暂停: {}", game_state.can_pause());
    
    game_state = GameState::Paused;
    println!("已暂停，可以恢复: {}", game_state.can_resume());
    
    // Shape 演示
    println!("\n4. 几何形状枚举演示:");
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
        Shape::Square { side: 5.0 },
    ];
    
    for shape in shapes {
        println!("{:?}", shape);
        println!("  面积: {:.2}", shape.area());
        println!("  周长: {:.2}", shape.perimeter());
        println!();
    }
}