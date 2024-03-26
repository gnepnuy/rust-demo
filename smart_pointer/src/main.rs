use crate::List::{Cons,Nil};


fn main() {

    // let b = Box::new(5);
    // println!("b = {}",b);

    // let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));

    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5,x);
    // assert_eq!(5,*y);
    // println!(" y = {}",y)
}


struct MyBox<T>(T);

impl 


enum List{
    Cons(i32,Box<List>),
    Nil,
}

