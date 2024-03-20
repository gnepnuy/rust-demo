use std::thread;
use std::time::Duration;

fn main() {
    // println!("Hello, world!");
    // simulated_expensive_calculation(10);

    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );


    // let x = 4;
    // let equal_to_x = |z|{
    //     z == x
    // };
    // let y = 2;

    // assert!(equal_to_x(y));

    // let x = vec![1,2,3];
    // let equal_to_x = move |z| z == x;
    // // println!("can't use x here:{:?}",x);
    // let y = vec![1,2,3];
    // assert!(equal_to_x(y));


    ////////// iterator

    // let v1 = vec![1,2,3];
    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}",val);
    // }


    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2,v1);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count:0}
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)|a * b)
        .filter(|x| x % 3 ==0)
        .sum();

    assert_eq!(18, sum);
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);

}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    
    assert_eq!(total,6);

}


struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}



fn generate_workout(intensity: u32, random_number: u32){
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_result = Cacher::new(expensive_closure);
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)

        );
    }else{
        if random_number == 3 {
            println!("Take a break today! Remenber to stay hydrated");
        }else{
            println!(
                "Today run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}