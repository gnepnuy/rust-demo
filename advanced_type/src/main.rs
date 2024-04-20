

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}





fn main() {

    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();


    // let answer = do_twice(add_one,5);

    // println!("The answer is: {}",answer);

    // let s1: &str = "Hello there!";
    // let s2: &str = "How's it going?";


    // type Kilometers = i32;
    
    // let x: i32 = 5;
    // let y: Kilometers = 5;
    
    // println!(" x + y = {}",x + y);


    // println!("Hello, world!");
}
