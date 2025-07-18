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








//traits-->consist only of method signatures
trait Animal{
    fn sound(&self)->String;//the trait defines this method
}
struct Sheep;
struct Cow;

impl Animal for Sheep{
    fn sound(&self)-> String{
        String::from("Maah")
    }
}
impl Animal for Cow{
    fn sound(&self)-> String{
        String::from("mooh")
    }
}
//passing a trait as a  parameter
pub fn notiify(item: &impl Summary){
    println!("breaking news {}", item.summerize());
}



//
struct Centimeters(f64);
struct Inches (i32);
impl Inches{
    fn to_centimeters(&self)-> Centimeters{
        let &Inches(inches)=self;
        Centimeters(inches as f64 * 2.54)
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct seconds(i32);
fn mn/*main*/(){
    let _one_second:Seconds=Seconds(1);
    println!("one second looks like: {:?}"/*debug notation */, _one_second);
    let _this_is_true = (_one_second==_one_second);
    let _this_is_true = (_one_second> _one_second);
    let foot : Inches = Inches(12);
    Println!("one foot equals {:?}", foot);
    let meter: Centimeters = Centimeters(100.0);
    let cmp: &str =
    if foot.to_centimeters() < meter {
        "smaller"
    } else{
        "bigger"
    };
    println!("one foot is {} than one meter")//output smaller
}


