fn main() {


    // let mut num = 5;

    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32; 

    // unsafe{
    //     println!("r1 is: {}",*r1);
    //     println!("r2 is: {}",*r2);
    // }


    // let address = 0x01223344usize;
    // let r = address as *count i32;


    // unsafe fn dangerous() {
    //     println!("i unsafe")
    // }

    // unsafe {
    //     dangerous();
    // }

    //创建不安全代码的安全抽象
    // let mut v = vec![1,2,3,4,5,6];

    // let r = &mut v[..];

    // let (a,b) = r.split_at_mut(3);

    // assert_eq!(a,&mut [1,2,3]);
    // assert_eq!(b,&mut [4,5,6]);

    // use std::slice;

    // let address = 0x01223344usize;
    // let r = address as *mut i32;

    // let slice: &[i32] = unsafe {
    //     slice::from_raw_parts_mut(r,10000)
    // };


    //使用 extern 函数调用外部代码

    // unsafe {
    //     println!("Absolute value of -3 according to C: {}",abs(-3));
    // }

    //访问或修改可变静态变量
    // println!("name is: {}",HELLO_WORLD);
    
    // add_to_count(3);

    // unsafe {
    //     println!("COUNTER: {}",COUNTER);
    // }

    //实现不安全 trait



}


unsafe trait Foo {

}

unsafe impl Foo for i32 {
    
}

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// static mut COUNTER: u32 = 0;

// static HELLO_WORLD: &str = "Hello, world!";

// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a rust function from c");
// }

// extern "C" {
//     fn abs(input: i32) -> i32;
// }


// use std::slice; 

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32],&mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();

//     assert!(mid <= len);
//     unsafe {
//         (
//             slice.from_rwa_parts_mut(ptr,mid),
//             slice.from_rwa_parts_mut(ptr.add(mid), len - mid)
//         )
//     }
// }