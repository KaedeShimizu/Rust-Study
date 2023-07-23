use rand::Rng;
use std::io;

// 使用一个枚举类型 有三个值，分别是小于大于等于
use std::cmp::Ordering;

fn main() {
    println!("猜数字！");

    let sectet_number = rand::thread_rng().gen_range(1..101);

    println!("神秘数字是{}", sectet_number);

    println!("猜测一个数字");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数字是{}", guess);

    // 这里同时需要进行类型矫正，这里的整数类型是i32，自动类型推断了，默认就是i32
    // 这里把字符串转换为数字
    // trim就是去掉两端的空白，比如空格和回车
    // parse是把字符串解析成某种数字类型，在声明变量的时候声明类型就好
    // 这个时候，前面的secretnumber自动变成u32了！
    // 最后面的expect就是报错，这里不做多说
    let guess: u32 = guess.trim().parse().expect("输入一个整数");

    // 比较大小
    // guess有一个方法，cmp，也就是compare比较，和传入的另外一个值进行比较
    // 接受的参数是另一个值的引用
    // match表达式是通过分支组成的，如果match后面跟着的值匹配了，那么就执行这个arm里面的代码
    match guess.cmp(&sectet_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
