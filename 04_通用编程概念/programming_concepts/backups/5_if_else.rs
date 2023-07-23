fn main() {
    // if表达式 就是根据条件执行不同的代码分支
    // 这个条件必须是bool
    // 与条件相关的就是分支，也就是arm
    // 也可以添加一个else表达式
    let n = 3;
    if n < 5 {
        println!("True");
    } else {
        println!("False");
    }

    // else if可以不断地嵌套 按照线性流程依次判断
    let n = 6;
    if n % 4 == 0 {
        println!("4");
    } else if n % 3 == 0 {
        println!("3");
    } else if n % 2 == 0 {
        println!("2");
    } else {
        println!("0");
    }

    // 如果使用了多于一个else if，那么代码就会很乱，这个时候因该用match重构
    // 同时，if是一个表达式，所以可以作为一个返回的内容
    let condition = true;
    // 这里就是直接放在等号右边了 这里要注意，两个数据类型必须一样，不能不一样
    let num = if condition { 5 } else { 6 };
    println!("num is {}", num);
}
