fn main() {
    // 有三种循环结构 loop while和for
    // loop 就是反复执行一个代码
    // 可以用break退出循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);

    // while 条件循环
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("END!");

    // for循环
    // 主要是遍历集合用的，效率高一点
    // 也可以用while和loop 但是容易出错，不方便
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    // 其实这就是一个迭代器，和Python差不多，可以对每个元素进行操作，不需要条件
    for i in a.iter() {
        println!("The val is {}", i);
    }

    // 和Python一样，有range这个东西，可以生成[1, 2)之间的数据
    // 不过不需要写range，直接写一个(x..y)就可以啦
    // 实现3，2，1倒计时
    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("DONE");
}
