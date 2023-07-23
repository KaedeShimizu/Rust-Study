// 源代码都应该放在src目录下面
// 如果没有使用cargo创建项目，直接把代码复制过来就好

// VS Code会自动编译，不要改.lock文件
// cargo run可以直接运行代码，非常快速
// cargo check会检查代码是否成立，不会产生可执行文件
// 如果后面加-release，那么就是发布程序了，这样子代码运行更快，但是比较慢
fn main() {
    println!("Hello, world!");
}
