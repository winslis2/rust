fn main() {
    //bool
    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true={}",is_true);
    println!("is_false={}",is_false);
    println!("Hello, world!");
    //char char是32位的，一般语言是8位的
    let a = 'a';
    println!("a={}",a);
    //i8,i16,i32,i64 f3，f64
    let b: f32 = 1.0;
    println!("数字b={}",b);
    //自适应类型isize  usize
    println!("max={}",usize::max_value());
    //数组
    let arr: [u32;5] = [1,2,3,4,5];
    println!("arr={}",arr[0]);//只能打印一个
    //元组
    let tup: (i32,char) = (11,'e');
    println!("tup={}",tup.0);
    //元组拆解
    let (x,y) = tup;
    println!("x={}",x);
    println!("y={}",y);

}

