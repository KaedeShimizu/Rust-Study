/* Vector
其实就当作数组就行了，只不过这个数组不需要设置大小，会自动根据实际需求进行修改
由标准库提供，类型必须相同，在内存里面是按照顺序存放的
可以用new创建
*/
fn main() {
    // 创建一个vector
    // 这里创建的是一个空的vec，所以需要指明数据类型
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // 当然，也可以用初始值创建，这里可以用一个宏
    let a = vec![1, 2, 3];
    println!("{:?}", a);

    // 添加元素
    // 如果只有下面这句话，那么就是错的
    let mut v = vec![];

    // 但是如果加上这个，rust就可以推断出来了，自动的就是i32
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    // 作用域一样，一旦离开作用域就会被清理掉了

    // 读取vector的元素
    // 可以用索引获取 这里用取地址的方法获取，防止清空vec
    let v = vec![1, 2, 3, 4, 5];
    // 获取第三个元素
    let third = &v[2];
    println!("{}", third);
    println!("{:?}", v);

    // 也可以通过vec的get方法获取
    // 这个更好进行防止非法访问报错
    match v.get(2) {
        // 能取出来就输出
        Some(third) => println!("The third is {}", third),
        // 没有就说没有
        None => println!("None"),
    }

    // 有一些借用上面的规则要注意
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); //cannot borrow `v` as mutable because it is also borrowed as immutable
    // 主要是防止报错用的，因为原来的内存可能会需要释放，但是引用不会改变
    println!("The first is {}", first);
    println!("v is {:?}", v);

    // 遍历Vec
    // 这里一般都用的是引用，我为了方便直接写在一块
    for i in &vec![1, 2, 3, 4, 5] {
        println!("{}", i);
    }

    // 可变引用
    for i in &mut vec![1, 2, 3, 4, 5] {
        // 在里面可以通过解引用然后修改原来的值
        *i += 50;
        println!("{}", i);
    }
}
