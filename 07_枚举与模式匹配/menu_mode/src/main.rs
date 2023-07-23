// if let学习
// if let就是一个简单的控制流，只关心一种匹配，忽略其他的情况
fn main() {
    let x = Some(5);
    // 我现在只看，等于3的情况
    if let Some(3) = x {
        println!("Three");
        // 如果不满足，也就是else，可以直接用_进行接受
    } else {
        println!("Not three");
    }
}
