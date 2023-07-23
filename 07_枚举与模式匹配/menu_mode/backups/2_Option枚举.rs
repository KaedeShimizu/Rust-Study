// option枚举
// 就是其它语言的Null 空
// 这个东西是预导入的
fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // Option<类型>其实和类型 不是同一种类型
    // 如果要使用T,那么必须要进行类型转换。
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    drop(x);
    drop(y);
}
