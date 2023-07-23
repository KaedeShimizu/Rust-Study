// 做一个vector的例子
// 这里需要存放不同类型的数据，那么就可以用到enum

// 首先创建一个枚举
enum IPID {
    V4(i8, i8, i8, i8),
    V6(String),
}
fn main() {
    // 创建一个vec，存放各种IP地址
    let address_list = vec![
        IPID::V4(127, 0, 0, 1),
        IPID::V6(String::from("jsk1hn323bn")),
        IPID::V4(127, 122, 1, 5),
    ];

    // 防止warn
    drop(address_list);
}
