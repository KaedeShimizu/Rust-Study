// 切片
fn main() {
    // 需求 找到输入的第一个单词，没有空格直接返回
    let mut s = String::from("Hello World");
    // 但是这样子一旦字符串改变，就无法保证有效性了
    let word_index = first_word(&s);
    s.clear();
    println!("{}", word_index);

    // 于是就有了字符串切片
    let s = String::from("Hello World");
    // 就是字符串一部分的引用 开始索引到结束索引，结束是终止位置的下一个索引值
    // 这里的0可以选择忽略，最后一个元素可以不管，如果就是字符串长度，也可以忽略
    let hello = &s[0..5];
    let world = &s[6..11];
    // 如果我要直接指向整个字符串，那么可以直接这样
    let whole = &s[..];
    println!("{} -- {}, {}", hello, world, whole);
    // 当然，字符串切片只能在utf-8范围内进行

    // 使用字符串切片进行处理
    let the_first_word = first_word_new(&s);
    // s.clear(); 这个时候如果改变，就会报错了，直接的避免类似的错误
    println!("{}", the_first_word);

    // 其实字符串字面值就是一个字符串切片
    // let s = "Hello World";

    // 使用字符串切片作为函数参数
    let my_string = String::from("Hello world");
    let word = first_word_new_2(&my_string[..]);
    println!("{}", word);

    // 其他类型的切片
    // 其实差不多，和Python也差不多
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    // 找到第一个空格
    // 通过s的一个方法，把字符串转换为字节数组
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // 判断
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 写一个改进版本
fn first_word_new(s: &String) -> &str {
    // 找到第一个空格
    // 通过s的一个方法，把字符串转换为字节数组
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // 判断
        if item == b' ' {
            // 如果是，返回一个切片，从开始到第一个空格
            return &s[..i];
        }
    }
    &s[..]
}

// 把字符串切片作为参数进行传递
// 前面的都是String类型，但是更好用的是切片
fn first_word_new_2(s: &str) -> &str {
    // 找到第一个空格
    // 通过s的一个方法，把字符串转换为字节数组
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // 判断
        if item == b' ' {
            // 如果是，返回一个切片，从开始到第一个空格
            return &s[..i];
        }
    }
    &s[..]
}
