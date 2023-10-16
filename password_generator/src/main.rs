use std::io;

fn main() {
    println!("Please select the mode Encrypt or decrypt");
    let mut mode=String::new();
    let mut plain_text=String::new();
    let mut coded_text=String::new();
    let mut iterations=String::new();
    io::stdin()
       .read_line(&mut mode);
   


    let mut mode=mode.trim().to_lowercase();    //Converts all characters to lowercase..removes the empty spaces and make the variable new for the the value used in the variable
    println!("You chose mode {}", mode);


   if mode == "encrypt"{
      
      println!("You chose encrypt");
      println!("Enter the plain text");
      println!("Enter code to code");
            io::stdin()
      .read_line(&mut plain_text);
             io::stdin()
            .read_line(&mut iterations);
   println!("The plain text is {} and code is {}", plain_text, iterations);
   
  
   }
   else  if mode=="decrypt"{
     println!("You choose decrypt");
      println!("Enter the coded text");
      println!("Enter the code to Decode");
     
      io::stdin()
      .read_line(&mut coded_text);
   io::stdin()
      .read_line(&mut iterations);
   
   println!("The plain text is {} and code is {}", coded_text, iterations);
   }

   else {

      println!("Wrong text");

   }
  
}
