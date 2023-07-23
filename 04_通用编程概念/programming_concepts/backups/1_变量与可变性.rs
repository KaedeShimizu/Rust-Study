// 常量需要用到const关键字，类型必须标注
// 常量只能直接指定，相当于cpp的Define X
// 命名规范：全部大写，单词之间用下划线分开
// 常量可以声明在任何作用于下
const MAX: u32 = 100_000;
fn main() {
    // 定义一个变量，这个时候变量不可变
    let x = 5;
    // 花括号是一个占位符
    println!("The value of x is {}", x);
    // 定义一个可变的变量
    let mut x = 5;
    println!("x is {}", x);
    x = 10;
    println!("x changed is {}", x);
    println!("MAX is {}", MAX);

    // 对于不可变变量，假设是y，可以通过隐藏，来改变这个y
    let y = 0;
    // 我现在让他+1，这个时候原来的y就被隐藏了
    // 这就是Shadowing
    // 用let关键字声明的同名新变量是不可变的，除非加上mut
    let y = y + 1;
    println!("y is {}", y);

    // 用let声明的变量，类型是可以变化的
    // 应该可以直接看出来类型的区别
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces)
}
