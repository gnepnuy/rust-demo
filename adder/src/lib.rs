pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a,2)
}

pub fn greeting(name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}",value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100,got {}",value);
        }

        Guess{value}
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}",a);
    10
}


fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn internal() {
        assert_eq!(4,internal_adder(2,2));
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10,value);
    }

    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(4,value);
    }
    #[test]
    #[ignore]
    fn it_works_v2() -> Result<(),String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),"Greeting did not contain name, value was `{}`",result);
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4,add_two(2));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn large_can_hold_smaller() {
        let large = Rectangle {width: 8, height: 7};
        let smaller = Rectangle {width: 5, height: 6};

        assert!(large.can_hold(&smaller));
    }
}




#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
