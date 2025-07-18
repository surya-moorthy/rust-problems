#[derive(Debug)]
struct User {
    name : String,
    last_name : String,
    age : u32
}

fn main() {
   
   let user1 = User {
     name : "Harkirat".to_string(),
     last_name : "Singh".to_string(),
     age : 19
   };

   println!("{:?}",user1);

}

