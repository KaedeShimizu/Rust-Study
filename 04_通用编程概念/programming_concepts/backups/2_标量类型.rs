fn main() {
    // 每个数据都有特定的数据类型
    // 这里说两个类型，标量和复合类型
    // 一般可以自动转换类型，但是如果类型比较多就需要添加类型标注
    // 否则直接报错
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // 标量，Rust只有四个：整数 浮点 布尔 字符
    // 整数类型
    // 就是没有小数的数据，无符号就是以u开头的，有符号就是i开头的
    // 分为8 16 32 64 128 arch，都分为有无符号，无符号其实就是非负数
    // i32 32就是占的位数
    // arch类型,就是系统架构 isize和usize就是根据计算机类型来看，使用的非常少

    // 整数的字面值
    // Hex 0xff；Octal 0o77；Binary 0b1111_0000 都可以添加下划线；Byte b'A'
    // 除了Byte类型，都允许类型后缀，比如57u8

    // 整数溢出
    // u8的范围是0~255，如果设置为256，那么就溢出了：
    // 调试模式下，一旦溢出，程序会发生panic（恐慌）
    // 发布模式下，Rust不会检测可能导致panic的溢出，如果溢出了，那就会环绕操作，比如256->0，257->1

    // 浮点类型
    // Rust就是有小数的浮点类型，分为两种，f32和f64，单精度和双精度
    // 默认是f64 所有操作都是f64
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x is {}, y is {}", x, y);

    // 布尔类型
    // 和其他一样，占用一个字节大小，有true和false
    let t = true;
    let f: bool = false;
    println!("t is {}, f is {}", t, f);

    // 字符类型
    // 使用单引号，占用四个字节的大小
    // 使用Unicode，可以表示任何字符，比如日文字符，中文，emoji之类的所有东西
    let x = 'z';
    let y: char = 'の';
    let z = '😃';
    println!("x is {}, y is {}, z is {}", x, y, z);
}
