/* String类型
其实就是基于字节的一个集合
并且有一些方法可以显示里面的内容
主要有一个叫做字符串切片的东西，或者引用切片
字符串字面值就是一种字符串切片
String类型，就是来自标准库了，用的是utf-8
Rust还有很多字符串类型，String结尾的字符串就是能获得所有权的，Str结尾的就是只能获取的了
*/
fn main() {
    // 创建一个新的字符串
    // 其实这个和Vec很像，比如可以用new方法
    // 但是这个是一个空字符串，用的少
    let s = String::new();
    drop(s);

    // 也可以从字符串字面值进行转换
    let s = "KaedeShimizu".to_string();
    println!("{}", s);

    // 再就是之前用的，用from方法创建
    let s = String::from("Jace");
    println!("{}", s);
    // 其实都差不多，可以根据喜好选择

    // 更新这个String
    // 附加一个字符串到String
    let mut s = "AAA".to_string();
    // 用push方法
    s.push_str("bar");
    println!("{}", s);
    // 这里传入的参数，使用完还是有所有权的，可以继续用
    let s1 = "Hello";
    s.push_str(s1);
    println!("{}, {}", s, s1);

    // 还有一个类似的方法，就是附加单个字符到字符串
    s.push('?');
    println!("{}", s);

    // 拼接字符串 直接用加号就好
    // 注意这里的s1是个切片，所以不需要引用
    let s3 = s + s1;
    println!("{}", s3);

    // 拼接后，s就不能使用了！因为使用加号其实是类似于add方法了

    // 或者可以用一个宏进行拼接
    // 类似于输出的函数，只是format是返回了本来应该输出的字符串，这个超级方便
    // 而且这个东西不会占用所有权的
    println!("{}", format!("{}-{}-{}", "Hello", "World", "Kaede"));
}
