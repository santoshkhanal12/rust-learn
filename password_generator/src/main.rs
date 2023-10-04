use std::io;

fn main() {
    println!("Please select the mode Encrypt or decrypt");
    let mut mode=String::new();
    let mut plain_text=String::new();
    let mut coded_text=String::new();
    io::stdin()
       .read_line(&mut mode);
    mode=mode.to_lowercase();    //Converts all characters to lowercase
    println!("You chose mode {}", mode);

   if mode=="encrypt"{
      println!("You chose encrypt");
      println!("Enter the plain text:");
            io::stdin()
      .read_line(&mut plain_text);
   println!("The plain text is {}", plain_text);
   }
   else  if mode=="decrypt"{
      println!("You choose decrypt");
      println!("Enter the coded text:");
      io::stdin()
      .read_line(&mut coded_text);
   println!("The plain text is {}", coded_text);
   }
  
}
