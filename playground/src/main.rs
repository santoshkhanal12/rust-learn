use std::io;
use std::any::type_name;

fn main() {
   println!("lets check if else condition");
   println!("Enter the string");
   let mut text=String::new();
   io::stdin()
   .read_line(&mut text);
println!("The entered string is {}", text);

if text==String::from("Encrypt"){
  println!("{}",text)
}


}
