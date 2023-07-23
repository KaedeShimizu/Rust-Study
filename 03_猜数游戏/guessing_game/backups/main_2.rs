use std::io;
// Rust有一个随机数的库，rand就可以啦
// 自己默认是没有的
// 你需要自己导包，在.toml文件里面进行修改
// 直接包名:版本就好

// 使用rand这个包
use rand::Rng; //这是一个treat，其实就是一个接口，定义了一些方法

fn main() {
    println!("猜数字！");

    // 定义一个变量，用来接收随机数
    // 这个数据不可变 这里方法更新了，用两个点点就好
    let sectet_number = rand::thread_rng().gen_range(1..101);

    // 输出这个数字
    println!("神秘数字是{}", sectet_number);

    println!("猜测一个数字");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数字是{}", guess)
}
