// 方法
// 先创建一个普通的结构体
struct Rect {
    width: u32,
    height: u32,
}

// 用关键字创建方法，必须有self参数，这里用引用，防止销毁传入的数据
// 必须要在impl块里面定义方法
impl Rect {
    // 这里的self自动推断为传入的Rect，当然，借用也可以是可变的
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 我要实现另外一个方法，直接继续写就好
    // 需求 判断一个长方形能否容纳另外一个长方形
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 创建一个关联函数
    // 比如创建一个正方形
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 方法不是函数，方法是上下文中定义的，第一个参数就是self，也就是调用的实例
    let rect = Rect {
        width: 20,
        height: 40,
    };
    // 调用这个方法，就不需要再写其他东西了，直接点就好
    println!("{}", rect.area());

    // 方法调用的运算符
    // Rust会自动进行引用或者解引用，从而匹配方法
    // 直接a.function(&b)就好

    // 方法参数
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    // 不管怎么样，直接某个结构点就好，后面根据报错实际需求决定
    println!("{}", rect1.can_hold(&rect2));

    // 关联函数，不把self作为第一个参数的函数，这就是关联函数
    // 类似于String::from()
    // 其实就类似于类方法吧
    let s = Rect::square(50);
    drop(s);

    // 同时，一个结构体可以有多个rmpl模块，就是分开写罢了
}
