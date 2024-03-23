use add_one as aa;
use add_two as bb;
fn main() {

    let num = 10;
    println!("Hello, world! {} plus one is {}!",num,aa::add_one(num));
    println!("Hello, world! {} plus two is {}!",num,bb::add_two(num));


}
