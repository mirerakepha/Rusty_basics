fn sum<T: std::ops::Add<Output = T>>(a:T, b:T) -> T{
    a+b
}
fn main(){
    assert_eq!(5, sum(2i8, 3i8));//i8
    assert_eq!(50, sum(20,30));//i32
    assert_eq!(2.46, sum(1.23, 1.23));//floating point .....f64
    println!("success");
}





//struct point
struct Point<T>{
    x:T,
    y:T,
}
fn nm/*main*/(){
    let integer: Point/*<i32>*/ = Point {x:5, y:10};
    let float: Point /*<f64>*/ = Point {x:1.0, y:4.0};
    println1("success");
}



//string 
struct Point<T, U>{
    x: T,
    y:U,
}
fn mn/*main */(){
    let p: Point<i32, String >= Point{x:5, y: "hello".to_string};
    println!("success");
}