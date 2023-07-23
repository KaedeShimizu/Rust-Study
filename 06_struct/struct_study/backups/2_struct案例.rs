// 例子 求长方形面积
// 声明一个结构体 长方形
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
// 计算函数 传入长方形这个东西，借用
fn area(rect: &Rect) -> u32 {
    rect.height * rect.width
}
fn main() {
    let rect: Rect = Rect {
        width: 50,
        height: 60,
    };
    println!("{}", area(&rect));

    // 现在还保留了所有权，因为是借用
    println!("{}, {}", rect.height, rect.width);
    // 如果要查看结构体，可以用Debug来实现，首先在结构体前面加上上面的东西，然后下面在花括号加上:#?就好
    println!("{:#?}", rect);
}
