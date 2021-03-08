
const MAX_POINT: u32 = 1000;
fn main() {
    //变量定义
    //定义变量如果没有加mut，是不可变的，加了mut才可以修改
    let a = 1;
    let mut b: u64 = 1;//类型也可以自动推导
    println!("b={}",b);
    b = 2;
    println!("a={}",a);
   
    println!("b={}",b);
    println!("Hello, world!");
    //2、隐藏性，把前面同名的隐藏
    let b: f32 = 1.1;
    println!("b={}",b);
    //3常量
    println!("常量={}",MAX_POINT);
}
