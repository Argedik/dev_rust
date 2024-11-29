use core::result::Result::Ok;

pub struct Conditions;

pub enum InputType {
  Boolean(bool),
  Numeric(f64),
  Unknown,
}
impl Conditions {
  pub fn check_type(input: &str) -> InputType {
    if let Ok(val) = input.parse::<bool>(){
      return InputType::Boolean(val);
    }

    if let Ok(val) = input.parse::<f64>(){
      return InputType::Numeric(val);
    }

    InputType::Unknown
  }

  pub fn check_type2 (input: &str) {
    if input == "12"{
      println!("{}", input)
    } else if input == "11" {
        println!("11")
    }
    
    let value_type = if input == "13" {println!("input değeri: {}", input)} else {println!("input değeri: {} değil", input)};
    println!("value tipi: {:?}", value_type)
  }
}

//aşağıdaki şekilde çağırılabilir
/*

mod modules;
use modules::control_flow::if_else::{Conditions, InputType};
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
*/