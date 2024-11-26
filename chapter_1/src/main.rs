// mod modules {
//   pub mod scope_checker;
//   pub mod input_reader;
//   pub mod explanation;
//   pub mod control_flow;
// }
mod modules;

use modules::control_flow::if_else::{Conditions, InputType};



fn main(){
    let enable_check = true;
    let input = "13";
    if enable_check {
        
        match Conditions::check_type(input){
            InputType::Boolean(val) => println!("'{}' is a boolean: {}", input, val),
            InputType::Numeric(val) => println!("'{}' is a boolean: {}", input, val),
            InputType::Unknown => println!("'{}' is a boolean:", input)
        }
    } else {
        println!("enable_check disabled!")
    }
    Conditions::check_type2(input);

    loop {
        println!("test"); // sınırsız döngü
    }
}