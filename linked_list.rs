use List::Nil;
use List::Cons;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn insert_at_beginning(self, elem : i32) -> List {
        Cons(elem, Box::new(self))
    }

    fn print(&self){
        match *self {
            Cons(num, ref tail) => {
                println!("{}", num);
                tail.print();
            }
            Nil => {
                println!("Done");
            }
        }
    }

    fn length(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => {
                1 + tail.length()
            }
            Nil => {
                0
            }
        }
    }
}

fn main(){
    let mut list = List::new();
    println!("Length at beginning: {}", list.length());
    list = list.insert_at_beginning(2);
    list = list.insert_at_beginning(52);
    println!("Length in middle: {}", list.length());
    list = list.insert_at_beginning(-2);
    list = list.insert_at_beginning(-3);
    println!("Length at end: {}", list.length());
    list.print();
    list.print();
}