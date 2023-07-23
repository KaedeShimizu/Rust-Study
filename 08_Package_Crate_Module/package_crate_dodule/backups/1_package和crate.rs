/* Rust的代码组织
主要包括哪些细节可以暴露，哪些细节是私有的，哪些名称有效
其实就是模块系统：
    Package 包 用来构建，测试，共享crate
    Crate 单元包 可以产生可执行文件或者library
    Module 模块 用来控制代码的组织
    Path 路径 为他们命名的一种方式
 */

/* Crate和Package
Crate的类型有：
    binary
    library
Crate Root
    就是源代码文件
    Rust编译器从这里开始，组成你的根Crate的根Module
Package
    包含一个Cargo.toml 描述了如何构建这个Crates
    只能包含0~1个library crate
    可以包含任意多个binary crate
    但是至少包含一个crate
 */

/* 创建一个package
其实创建项目的时候，默认就是一个package文件了，你可以看到这些文件
src/main.rs
    默认就是binary crate的crate root
    crate名称和包名称相同
src/lib.rs
    packag包含一个library crate
    library crate的root
    crate名称和包名相同
 */

/* 补充
一个包可以同时包含上面那两个文件
这代表这个包有一个二进制crate个一个library crate
 */

/* Crate的作用
将相关功能组合到一个作用域内，便于在项目间进行共享，防止冲突
比如rand 就要通过名称进行访问
 */

/* 定义Module来控制作用域和私有性
Module
    在一个crate里面，把代码进行分组
    增加可读性，易于复用3
    控制项目的私有性

建立一个Module
    使用mod关键字
    可以嵌套
    可以包含其他项目的定义，比如函数之类的
    看到这里请打开lib.rs文件
 */
fn main() {
    println!("Hello, world!");
}
