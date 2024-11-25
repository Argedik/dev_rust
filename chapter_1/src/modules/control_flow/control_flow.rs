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
}

