

// Option and Result enum

// return null , none , nil --> Option
// error handling --> Result

use std::{fmt::Error, fs};


fn find_first_a(s : String) -> Option<i32> {
   for (index,char) in s.chars().enumerate() { 
     if char  == 'a'{
        return Some(index as i32);
   };
}
 return None;
}

fn read_file(filepath : String) -> Result<String,Error> {
  let greeting_sir = fs::read_to_string(filepath);

  match greeting_sir {
    Ok(string) => {
        println!("{}",string);
      Ok(string)
     },
    Err(err) => {
        println!("there is a problem while accessing the file : {}",err);
       Ok(err.to_string())
    }
  }
}


fn main() {
   let index = find_first_a("hello world".to_string());
   match index {
      Some(a) => {
        println!("the index is {}",a);
      }
      None => {
        println!("No 'a' in the given string");
      }
   }

}