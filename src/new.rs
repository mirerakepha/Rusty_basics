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


