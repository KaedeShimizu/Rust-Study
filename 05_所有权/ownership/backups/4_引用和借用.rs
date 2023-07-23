// 引用和借用
fn main() {
    // 创建一个字符串
    let s1 = String::from("Hello World");
    // 这里的&s1，不拥有s1，就是一个引用，所以走出作用域后不会清理s1
    let len = get_length(&s1);
    println!("The length of \"{}\" is {}", s1, len);

    // 可变的引用
    let mut s1 = String::from("Hello");
    // 都需要可变，所以用mut修饰引用
    let len = get_length2(&mut s1);
    println!("The length of '{}' is {}", s1, len);

    // 当然有一个限制，特定的作用域内，只能有一个可变的引用
    // 多个就报错啦，这是为了防止数据竞争
    // 如果不在同一个作用域，那么可以创建一个变量的多个可变引用

    // 同时，不能同时拥有一个可变引用和一个不可变引用，但是多个不可变引用是可以的

    // 悬空引用 Dangling References
    // 简单说就是一个指针引用了某个地址，但是这个地址已经释放或者给别人用了
    // Rust可以保证不会出现这样的情况
}

// 写一个返回大小的函数
// 用引用的方法来写，其实就是取地址符，和cpp差不多
// 把引用作为函数参数的行为，就叫做借用
fn get_length(s: &String) -> usize {
    // 不能修改借用的东西，默认不可变
    // s.push_str(", world");
    s.len()
}

fn get_length2(s: &mut String) -> usize {
    // 写一个通过引用修改的例子
    s.push_str(", world");
    s.len()
}

// 给一个悬空引用的函数
// missing lifetime specifier
// fn dangle() -> &String{
//     let s = String::from("Hello");
//     &s
// }

/*
总结
在任何情况，只能有一个可变的引用或者有n个不可变的引用
引用必须一直有效
 */
