fn main() {
    println!("Hello, world!");

let x : i32 = 5;
let _y : i32; //adding an underscore fixes tthe unused variable y

assert_eq!(x,5);
println!("success!"); //therefore prints success

#[allow(unused_variables)]//this makes: does not bring a warning even if a variable is not used



//double quotes for string
//single quotes for char

fn xy(){

let mut x = 1;
x = x + 2; //this can also be written as x =+2

assert_eq!(x, 3);
println!("success!");

}








fn xy_print(){
    let x = 10;
let y = 5;
{
    
    println!("the value of x is {} and the value of y is {}", x, y);
}
println!("the value of x is {} and the value of x is {}", x, y);
}





fn helloworld(){
    define_x();
}
fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x)
}






//declare the same variable but different values 
fn dclr_twovar(){
    let x = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x)  //this prints 42
}






//shadowing and rebinding
fn rebind(){
    let mut x = 1;
    x = 7;
    //shadow and rebind
    x = x + 3;


    let y = 4;
    //shadowing
    let y: &str = "i bound text too";
    println!("success");
}








//updating an initialized variable
fn update() {
    let (mut x, y) = (1, 2);
    x += 2;   //can also be written as x = x + 1
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("success");
    
}





//declare 2 variables in the same line
fn vaar() {
    let (x, y);
    (x,..) = (3, 4);
    [..,y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    println!("success");
    
}






//assign x to y 
fn ass(){
    let x = 5;
    let mut y = 5;

    y = x;

    let z = 10;
    println!("success");

}


//assigning same type 
fn asign_bit() {
   let v: u16 = 38_u8 as u16 ; //38 was originally of type u8
   println!("success");

}



//return the type of a variiable
fn rett(){
    let x: u32 = 5;
    assert_eq!("u32" .to_string(), type_of(&x));

    println!("success");

}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>()) //prints u32
}








//MAX returns the largest number thet can be represented by the type
fn maxi() {
    assert_eq!(i8::MAX, 127); //assigned 8 bit integer
    assert_eq!(u8::MAX, 255); //unassigned 8 bit integer

    println!("success");
    
}





//Numbers in different number systems
fn numsys() {

    //the underscores dont afffect the actual value....its for readability
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //numbers with different number systems
    assert!(v == 1579); //result of 1024 + 255 + 63 + 255

    println!("success");
    
}








// floating point value is too precise
fn fl_point() {
    //assign f32 which is less precise(0.1 + 0.2)
    assert!(0.1 as f32+ 0.2 as f32 == 0.3 as f32); //the answer can be 0.3000000000002 not exactly 0.3
    
}






//
use std::ops::{Range, RangeInclusive};
fn range_() {
    assert_eq!((1..5), Range{start:1, end:5}); //prints the range with 5 excluded
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); //prints the range 5 included

    println!("success");    
}





//size of characters in binary
fn sizebn() {
    let c1: char = 'a';
    println!("{}", size_of_val(&c1)); //prints 4 (no of bytes)
    assert_eq!(size_of_val(&c1), 1);
    
    let c2: char = '@';
    assert_eq!(size_of_val(&c2), 4);


    println!("sucess");
    
}








fn boolprnt() {
    let _f: bool = false;
    let t = true;
    if !t {                  //checks if the condition is true 
        println!("success");
        
    }
    let f:bool = true && false ; //prints false
    
}






//diverging functions
fn dvrg() {
    println!("success");
    
}

fn get_option(tp: u8) -> Option <i32>{
    match tp {1 => {
        //if tp was 1 then this codeblock is executed
    } _ => {
        //anything else other than 1 this codeblock is executed 
    }};

    never_return_fn(); //this doesnt return to the caller  
}
fn never_return_fn() {
    
}
//implement the function
 /*fn never_return_fn() -> !{
     panic!() }*/




//deep copy
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2)










         
}
















//comment


