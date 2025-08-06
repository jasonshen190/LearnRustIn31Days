#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enums() {
    println!("=== 枚举基础 ===");
    
    // 简单枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4 类型: {:?}", four);
    println!("IPv6 类型: {:?}", six);
    
    // 带数据的枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("本地地址: {:?}", home);
    println!("回环地址: {:?}", loopback);
    
    // 使用函数处理枚举
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    process_ip_address(home);
    process_ip_address(loopback);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由到 IPv4 地址"),
        IpAddrKind::V6 => println!("路由到 IPv6 地址"),
    }
}

fn process_ip_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("处理 IPv4 地址: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("处理 IPv6 地址: {}", addr);
        }
    }
}