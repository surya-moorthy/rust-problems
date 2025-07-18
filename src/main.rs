fn main() {
  let string = String::from("value");
  let ans = get_string_length(&string);
}

fn get_string_length(s : &str) -> usize {
    s.chars().count()
}