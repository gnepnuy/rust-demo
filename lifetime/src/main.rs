fn main() {
    // println!("Hello, world!");

    // let string1 = String::from("long string is long");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(),string2.as_str());
    //     println!("The longest string is {}",result);
    // }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(),string2.as_str());
    // }
    // println!("The longest string is {}",result);

    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt {part: first_sentence};
}


struct ImportantExcerpt<'a> {
    part: &'a str,
}

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }




// fn longest<'a>(x: &'a str,y: &str) -> &'a str {
//     x
// }


// fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }