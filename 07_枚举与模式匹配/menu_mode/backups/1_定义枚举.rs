/*
枚举 就是列举所有可能的值，然后定义
enum非常强的！
 */

// 定义一个枚举
enum IpAddKind {
    // 可以把数据附加到枚举的变体当中
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // 没有关联数据
    Quit,
    // 包含了一个匿名结构体
    Move { x: i32, y: i32 },
    // 一个字符串
    Write(String),
    // 关联三个i32数据
    ChangeColor(i32, i32, i32),
}

// 枚举也可以定义方法
impl Message {
    fn call(&self) {}
}
fn main() {
    // 创建枚举值，这里直接把数据附加上去
    let four = IpAddKind::V4(127, 0, 0, 1);
    let six = IpAddKind::V6(String::from("::1"));

    // 使用枚举值
    route(four);
    route(six);

    // 可以分别定义
    let g = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 0, 0);

    // 都可以使用方法
    g.call();
    m.call();
    w.call();
    c.call();
}

fn route(ip_kind: IpAddKind) {
    drop(ip_kind);
}
