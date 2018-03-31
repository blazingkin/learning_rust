#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main(){
    let name = "Alex";
    let age = 20;
    let alex = Person { name , age};
    println!("{:#?}", alex);
}