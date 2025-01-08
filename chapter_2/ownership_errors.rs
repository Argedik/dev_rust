
//Hatalı kod için çözüm önerileri
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// 1
// pub fn return_a_string() -> String {
//     let s = String::from("Hello world");
//     s
// }

// 2
// fn return_a_string() -> &'static str {
//     "Hello world"    
// }

// 3
// use std::rc::Rc;
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello world"));
//     Rc::clone(&s)
// }

// 4
// fn return_a_string(output: &mut String) {
//     output.replace_range(.., "Hello world");
// }
//https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html