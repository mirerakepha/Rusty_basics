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