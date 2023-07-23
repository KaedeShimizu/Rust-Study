use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字！");

    let sectet_number = rand::thread_rng().gen_range(1..101);

    println!("神秘数字是{}", sectet_number);
    // 需要让用户不断地输入，就需要用到循环，loop
    // 注意缩进，虽然问题不大
    loop {
        println!("猜测一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜测的数字是{}", guess);

        // 这里parse会崩溃，如果输入的不是一个数字的话
        // 这里可以用match表达式进行替换
        // match进行错误处理是一个非常常用的方法
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     //如果成功，解析出来就是一个数字，直接返回这个数字就好
            Err(_) => continue, //如果输入的不是数字，那就直接进入下一次循环
        };
        match guess.cmp(&sectet_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            // 为了让程序停止，需要退出这个程序，其实就是break
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
