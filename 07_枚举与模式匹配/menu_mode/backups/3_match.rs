// match 是一个控制流运算符
// Rust提供了这个东西，允许一个值和一堆模式进行匹配

#[derive(Debug)]
enum USA {
    Alabama,
    Alaska,
}

// 先定义一个枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USA),
}

// 给一个函数，根据枚举的变体，把具体的数字返回回来
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // 会按照顺序进行匹配 这里简单，所以可以直接箭头到一个数值
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        }
        // 可以使用下划线通配符，如果是其他东西的话
        _ => 50,
    }
    // 其实match就相当于switch吧，每个都会走一遍罢了
}
fn main() {
    // 绑定值的模式
    let coin = Coin::Quarter(USA::Alabama);
    println!("{}", value_in_cents(coin));
}
