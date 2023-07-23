// 定义并实例化struct 也就是结构体
// 这是一种自定义数据类型，可以把一堆值给成一个结构
// 结构体的每个字段都需要名称和类型，用逗号分隔，最后一个也有逗号
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 给一个函数，用来创建User，返回一个user对象
fn build_user(email: String, username: String) -> User {
    User {
        // 这里因为username和email两个字段名称一样，可以简写
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    // 实例化这个结构体 其实类似于class
    // 每个字段都需要赋值
    let user1 = User {
        email: String::from("kaedeshimizu@163.com"),
        username: String::from("Kaede"),
        active: true,
        sign_in_count: 550,
    };

    // 访问直接通过点标记法就好
    println!("{}", user1.email);

    // 结构体可以作为一个返回值进行返回
    let kaede = build_user(String::from("kaedeshimizu@163.com"), String::from("Kaede"));
    println!(
        "{},{},{},{}",
        kaede.username, kaede.email, kaede.sign_in_count, kaede.active
    );

    // 如果要基于某个结构体，创建一个新的结构体，可以用更新语法
    let user2 = User {
        // 不同的正常写
        email: String::from("aaa@163.com"),
        username: String::from("aaa"),
        // 剩下的相同，直接用两个点点解构就好
        // 但是这个后面就不能有逗号了
        ..user1
    };
    drop(user2);

    // 特殊类型的结构体 Tuple Struct
    // 里面的元素是没有名称的
    // 比如这样子
    struct Color1(i32, i32, i32);
    struct Color2(i32, i32, i32);
    let black = Color1(0, 0, 0);
    let orange = Color2(0, 0, 0);
    // 这里的black和orange虽然说一样，但是是不同的类型，因为属于不同结构体的实例
    drop(black);
    drop(orange);

    // 空结构体 没有任何字段的结构体
    // 就是在某个类型实现一个trait的时候，可以用到这个东西

    // 结构体中的所有权
    // 具体要到后面的生命周期，就是保证结构体里面的引用有效
}
