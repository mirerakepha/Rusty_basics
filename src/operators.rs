use std::ops;

fn multiply<T: std::ops::Mul<Output= T>>(a:T, b:T)-> T{
    a*b
}

fn main(){
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
    println!("success");
}





//Foo + Bar -> Foo.add(Bar)
struct Foo;
struct Bar;

#[derive(PartialEq, Debug)]
struct FooBar;

#[derive(PartialEq, Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo{
    type Output = FooBar;
    fn add(self, _rhs: Bar)-> FooBar{
        FooBar
    }
}
impl ops::sub<Foo> for Bar{
    type Output = BarFoo;
    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}
fn mn/*main */(){
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    println!("success");
}







//associated types
struct MyStruct{}
impl MyTrait for MyStruct {
    type MyType = i32;

    fn get_my_type(&self)->Self::MyType{
        return 42
    }
}




//
trait bird{
    fn quack(&self)->string;
}
struct Duck;
impl Duck {
    fn swim(&self){
        println!("look, the duck is swimming");
    }
}
struct Swan;
impl Swan{
    fn fly(&self){
        println!("look, the swan is flying");
    }
}
impl Bird for Duck{
    fn quack(&self)-> String{"duck duck".to_string()}
}
impl Bird for Swan{
    fn quack(&self)-> String{"swan swan".to_string()}
}
fn mnn /*main */(){
    let duck: Duck = Duck;
    duck.swim();

    let bird:Box<dyn Bird> = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird:Box <dyn Bird> = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");
    println!("sucess");
}
fn hatch_a_bird(species: u8)-> Box <dyn Bird>{
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!()
    }
}





//implementing traits
trait Draw {
    fn draw(&self) ->String;
}
impl Draw for u8 {
    fn draw(&self)-> String {
        format!("u8: {}", self)
    }
}
impl Draw for f64 {
    fn draw(&self)-> String {
        format!("f64: {}", self)
    }
}
fn main(){
    let x: f64 = 1.1f64;
    let y: u8 = 8u8;

    draw_with_box(Box::new(x));//box x
    draw_with_ref(&y);
    println!("success");
}
fn draw_with_box(x: Box<dyn Draw>){
    x.draw();
}
fn draw_with_ref(x: Box<dyn Draw>){
    x.draw();
}
