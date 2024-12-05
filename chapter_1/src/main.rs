mod modules {
  pub mod scope_checker;
  pub mod input_reader;
  pub mod explanation;
}

fn main(){
  // modules::scope_checker::scope_checker::check_scope_behavior();
  // modules::input_reader::input_reader::read_and_print_input();
  modules::explanation::explanation::explain_array_indexing();
}