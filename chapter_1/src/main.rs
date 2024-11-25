// mod modules {
//   pub mod scope_checker;
//   pub mod input_reader;
//   pub mod explanation;
//   pub mod control_flow;
// }
mod modules;

use modules::control_flow::control_flow::{Conditions, InputType};



fn main(){
    let enable_check = true;

    if enable_check {
        let input = "12.312.12,3"; 
        match Conditions::check_type(input) {
            InputType::Boolean(val) => println!("'{}' is a boolean: {}", input, val),
            InputType::Numeric(val) => println!("'{}' is a numeric value: {}", input, val),
            InputType::Unknown => println!("'{}' is of unknown type.", input),
        }
    } else {
        println!("Input check is disabled.");
    }
}