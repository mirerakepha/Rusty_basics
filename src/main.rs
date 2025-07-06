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

println!("s1 = {}, s2 = {}", s1, s2);




//
fn strr() {
    let s: String = String::from("hello world");

    print_str(s.clone());

    println!("{}", s);

    
}
fn print_str() {
    println!("{}",s)
    
}






// Box allows you to allocete directly on a heap
fn bx() {
    let x:Box<i32> = Box::new(5);
    let mut y:Box<i32> = Box::new(1) ;

    *y = 4; //dereferences since you cannot direclty allocate 4 to y

    assert_eq!(*x, 5);
    println!("success");
    
}





//partial move
fn person() {
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let persn: Person = Person{
        name: String::from("alice"),
        age: Box::new(20),
    } ;

    let Person {name, ref age} = persn;
    println!("the person's age is {}", age);
    println!("the person's name is {}", name);//now owner of the string alice
}







//print the memory address
fn mem() {
    let x = 5;
    let p: &i32 = &x ;//pointer ==>holds the memory address

    println!("thememory address of x is{:p}", p);
    //the memory address of x is 0x7ffc0965a37c---> it is where the value 5 is stored
    
}




//replacing a substring
fn sbstr() {
    let mut s: String = String::from("i hate dogs") ;//the s should be mutable

    let s1 = s.replace("dogs", "cows");//dogs changes to cows

    assert_eq!(s1, "i hate cows");
    println!("success");
    
}







//convert &str to string
fn sstr() {
    let s: &str = "hello, world" ;
    greetings(String::from(s)) //convert &str--> string
}
fn greetings(s: String) {
    println!("{}", s);
    
}




//convert string to &str
fn sttr() {
    let s: String = "hello, world".to_string();
    let s1: &str = &s; //string to &str
    
    println!("success");
}



//string index
fn strid() {
    let s1 = String::from("hi,@#") ;
    let h: &str = &s1[0..1];//access h (hi)
    assert_eq!(h, "h");

    let h1 = &s1[3..6];//excludes 6
    assert_eq!("{}",h1);

    println!("success");

    
}





//initializing all elements to the same value at once
fn intarr() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("success");
    
}




//slice ==> borrows part of a collection without taking ownership of the entire collection
fn slc() {
    let arr = [1, 2, 3];
    let s1 :&[i32]= &arr[0..2];
    let s2: &str = "hello world";

    println!("success") ;  
}






//
fn complexst() {
    let s: &str = "你好，世界";    

    let slice: &str = &s[0..3];

    assert!(slice == "你" );

    println!("success");
}






















         
}
















//comment


