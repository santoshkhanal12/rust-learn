use std::io;

fn coding(plain_text:String,iterations: i32){
   let mut count=0;
   while count<=iterations{
      println!("Print {} iterations",count);
     for letter in plain_text.chars(){
      println!("{}",letter);
     }

     count=count+1;
   } 
   
}

fn decoding(coded_text: String,iterations: i32){
   let mut count=0;
   while count<=iterations{
      println!("Print {} iterations",count);
      for letter in coded_text.chars(){
         println!("{}",letter);
     
     } 
     count=count+1;
   }
}

fn main() {
    println!("Please select the mode Encrypt or decrypt");
    let mut mode=String::new();
    let mut plain_text=String::new();
    let mut coded_text=String::new();
    let mut iterations=String::new();
   
    io::stdin()
       .read_line(&mut mode);
   


       let mut mode=mode.trim().to_lowercase();     //Converts all characters to lowercase..removes the empty spaces and make the variable new for the the value used in the variable
    println!("You chose mode {}", mode);
    

   if mode == "encrypt"{
      
      println!("You chose encrypt");
      println!("Enter the plain text");
      println!("Enter code to code");
            io::stdin()
      .read_line(&mut plain_text);
             io::stdin()
            .read_line(&mut iterations);
         let iterations: i32 = iterations.trim().parse().unwrap();
         let mut plain_text=plain_text.trim().to_lowercase();
   println!("The plain text is {} and code is {}", plain_text, iterations);
   coding(plain_text,iterations)

   }
   else  if mode=="decrypt"{
     println!("You choose decrypt");
      println!("Enter the coded text");
      println!("Enter the code to Decode");
     
      io::stdin()
      .read_line(&mut coded_text);
   io::stdin()
      .read_line(&mut iterations);
   let iterations: i32 = iterations.trim().parse().unwrap();
   let mut coded_text=coded_text.trim().to_lowercase();
   println!("The coded text is {} and code is {}", coded_text, iterations);
   
   decoding(coded_text,iterations);
   
   
   }
   
   else {

      println!("Wrong text");

   }
  
}
