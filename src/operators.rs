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
