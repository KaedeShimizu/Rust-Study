// 定义一个饭店前台的mod
pub mod front_of_house {
    // 定义子mod
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 声明一个函数
pub fn eat_at_restaurant() {
    // 我想要调用上面的函数
    // 通过绝对路径
    // 这里加一句，为了访问上面的东西，需要加上pub，公开属性，不然访问不到
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 定义一个其他的模块
mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // 进入外界目录，调用外面的函数
        super::serve_order();
    }

    // 可以把结构体声明为一个公共的
    // 这个时候虽然结构体公共，但是字段是私有的
    pub struct breakfast {
        // 一个私有字段：
        time: String,
        // 一个共有字段：
        pub name: String,
    }

    // 枚举是一样的
    pub enum App {
        // 里面的所有变体都是公共的了，不需要加pub
        // 因为只有公共的才能用
        Soup,
        Salad,
    }
}

// 模块外面放一个函数
fn serve_order() {}

// use使用模块 使用绝对路径
use crate::front_of_house::hosting;

// 然后就可以正常使用了
// 当然，里面的函数要是共有的
pub fn abaaba() {
    hosting::add_to_waitlist();
}

// 相对路径也是可以的
use back_of_house::App;

// 当然，可以直接引入函数，但是为了方便阅读，一般引入的是函数的父级模块
// 不然就看不懂了
// 但是对于enum之类的东西，直接到本身就好
// 如果是同名条目，指定到父级就好

// as关键字，其实就是和Python一样了
use front_of_house::hosting as host;