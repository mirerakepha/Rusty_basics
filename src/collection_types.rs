//string
//std::string::String
fn mainn (){
    let mut s: String = String::from("hello world");

    let slice1: &str = s.as_str();
    assert_eq!(slice1, "hello world");

    let slice2: &str = &s[..5];
    assert_eq!(slice2, "hello");

    let mut slice3: String = s;
    slice3.push('!');
    assert_eq!(slice3, "Hello world");

    println!("success");
}


//indexing
fn main(){
    let s: String = String::from("hello, 水笔");
    let slice1: &str = s[..1];//h only takes one byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2: &str = [7..10];
    assert_eq!(slice2, "水");

    for (i,c ) in s.chars().enumarate() {
        if i == 7{
            assert_eq!(c, "水");
        }
    }
    }