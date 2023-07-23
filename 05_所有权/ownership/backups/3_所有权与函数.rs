/*
把值传递给函数和把值赋给变量是类似的
会发生复制或者移动
 */
fn main() {
    let s = String::from("Hello world");
    // 这里直接传入以后，s就不能再次使用了
    take_ownership(s);
    // println!("s is {}", s); value borrowed here after move 报错

    let x = 5;
    makes_copy(x);
    println!("x is {}", x);

    // 返回的时候所有权也会发生转移
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    // s2被销毁了，无法使用
    let s3 = takes_and_gives_back(s2);
    println!("{}, {}", s1, s3);

    // 如果想让函数使用一个值，但是不获取对于的所有权呢？
    // 有一个东西叫做引用
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
