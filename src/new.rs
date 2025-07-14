enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main(){
    let msg: Message = Message::Move{X: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("never let this run");
    }
    println!("success");
}
//output "success"





fn Main(){
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 0)
    ];
    for msgg in msgs{
        show_message(msg)
    }
}
fn Show_message(msg: Message){
    println!("{}", msg); 
}





//
fn option(){
    let five: Option<i32> = Some(5);
    let six:option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(none);

    if let Some(n) = six {
        println!("{}", n);
        println!("success!");
    }
    else{
        panic!("never run this")
    }
}
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}





//
fn itere(){
    let a: [i32; 4] = [4,3,2,1];

    for (i,v) in a.iter().enumerate(){ //enumerator method
        println!("the {}th item is {}", i+1, v);
    }
}
//the 1th element is 4
//the 2th element is 3
//the 3th element is 2
//the 4th element is 1









// while loop
fn loopw(){
    let mut n: i32 = 1;
    while n <10 {
        if n % 15 == 0{
            println!("Rizz");
        }
        else if n % 3 == 0{
            println!("fizz");
        }
        else if n % 5 == 0{
            println!("buzz");
        }
        else {
            println!("{}");
        }

        n += 1;
    }

    println!("n reached {}, loop over", n);

}





//looping and breaking
fn coount(){
    let mut counter: i32 = 0;
    let result: i32 = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    println!("success");
}






//using match to get the data an enum variant holds
enum message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn booln(){
    let msgs:[Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    println!("success");
}
fn show_message(){
    match msg{
        Message::Move {x:a, y:b}=> {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g,b)=> {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")//if nothing matches then it prints no data in the element
    }
}




//
enum Foo{
    Bar,
    Baz,
    qux(u32)
}
fn mn(){
    let a:: Foo = Foo::Qux(10);
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}








//vector => a vector is dynamic in size
enum MyEnum{
    Foo,
    Bar
}
fn eBar(){
    let mut count = 0;
    let v: Vec<MyEnum>= vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 2);
    println!("success");
}



//patterns
// 10| 20| 30 => means 10 or 20 or 30
//0..=5 => means from 0 to 5 (5 included)


//match guard
fn mtch(){
    let num: Option<i32> = some(4);
    let split: i32 = 5;
    match num { 
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
    println!("success");
}




 //methods.....defined within the context of struct unlike functions
 struct Rectangle{
    width: u32,
    height: u32,
 }
 impl Rectangle {
    fn area(self) -> u32{
        self.width * self.height 
    }
 }
 fn mn/*main*/(){
    let rect1: Rectangle = rectangle {width:30, height:50};
    assert_eq!{rect1.area(), 1500};
    println!("success");
 }




 //associated functions
 struct TrafficLight{
    color: String,
 }
impl TrafficLight{
    //return a trafficColor contains color "red"
    pub fn new()-> self{
        self{
            color: String::from("red"),
        }
    }
    pub fn get_state(&self) -> &str {
        &self.color
    }
}
fn mainn /*main*/(){
    let light: TrafficLight = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    println!("success")
}
