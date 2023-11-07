// fn main1() {
//     println!("Hello, 🌏!");
//     println!(r#"<a href="link.html">link</a>"#);
//     println!("<a href=\"link.html\">link</a>");
//     println!("{:?}",b"abc");
//     println!("{:?}",&[97,98,99]);

//     let mut a:[i8;10] = [42;10];
//     a[5] = 0;
//     println!("a:{:?}",a);
//     println!("a:{:#?}",a);

//     let t:(i8,bool) = (7,true);
//     println!("t.0:{}",t.0);
//     println!("t.1:{}",t.1);
// }

// fn main(){
//     // mut 为可变引用
//     let mut x : i32 = 10;
//     let ref_x: &mut i32 = &mut x;
//     *ref_x = 20;
//     println!("x:{x}")
// }

// fn main(){
//     let ref_x: &i32;
//     {
//         let x:i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x:{ref_x}")
// }

// fn main() {
//     // 切片
//     let  a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     println!("a: {a:?}");

//     let s: &[i32] = &a[2..5];
//     println!("s: {s:?}");
// }

// fn main(){
//     // String 和 str 的区别
//     let s1: &str = "World";
//     println!("s1:{s1}");

//     let mut s2:String = String::from("Hello ");
//     println!("s2:{s2}");
//     s2.push_str(s1);
//     println!("s2:{s2}");

//     let s3: &str = &s2[6..];
//     println!("s3:{s3}");
// }

// fn main(){
//     print_fizzbuzz_to(1000)
// }

// fn print_fizzbuzz_to(n:u32){
//     for i in 1..=n{
//         println!("{}",fizzbuzz(i));
//     }
// }

// fn fizzbuzz(n: u32) -> String{
//     let fizz = if is_divisible(n,3) {"fizz"} else {""};
//     let buzz = if is_divisible(n, 5){"buzz"} else {""};
//     if fizz.is_empty() && buzz.is_empty(){
//         return format!("{n}");
//     }
//     format!("{fizz}{buzz}")
// }

// /// 文档化
// fn is_divisible(n:u32,divisor:u32) -> bool{
//     if divisor == 0{
//         return false;
//     }
//     n % divisor == 0
// }



fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x as i16, y));
}
