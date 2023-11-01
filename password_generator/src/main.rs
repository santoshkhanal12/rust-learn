use std::io;

fn coding(mut plain_text:String,iterations: i32){
   let mut count=0;
   let mut encoded_text: String = "".to_owned();

   while count<iterations{
      println!("Print {} iterations",count);
     for letter in plain_text.chars(){ 
     if letter=='a'{
      let mut addcoded: &str="a";
      encoded_text.push_str(addcoded);
     }
     if letter=='b'{
      let mut addcoded: &str="b";
      encoded_text.push_str(addcoded);
     }
     if letter=='c'{
      let mut addcoded: &str="f";
      encoded_text.push_str(addcoded);
     }
     if letter=='d'{
      let mut addcoded: &str="k";
      encoded_text.push_str(addcoded);
     }
     if letter=='e'{
      let mut addcoded: &str="g";
      encoded_text.push_str(addcoded);
     }
     if letter=='f'{
      let mut addcoded: &str="c";
      encoded_text.push_str(addcoded);
     }
     if letter=='g'{
      let mut addcoded: &str="d";
      encoded_text.push_str(addcoded);
     }
     if letter=='h'{
      let mut addcoded: &str="h";
      encoded_text.push_str(addcoded);
     }
     if letter=='i'{
      let mut addcoded: &str="l";
      encoded_text.push_str(addcoded);
     }
     if letter=='j'{
      let mut addcoded: &str="p";
      encoded_text.push_str(addcoded);
     }
     if letter=='k'{
      let mut addcoded: &str="u";
      encoded_text.push_str(addcoded);
     }
     if letter=='l'{
      let mut addcoded: &str="q";
      encoded_text.push_str(addcoded);
     }
     if letter=='m'{
      let mut addcoded: &str="m";
      encoded_text.push_str(addcoded);
     }
     if letter=='n'{
      let mut addcoded: &str="i";
      encoded_text.push_str(addcoded);
     }
     if letter=='o'{
      let mut addcoded: &str="e";
      encoded_text.push_str(addcoded);
     }
     if letter=='p'{
      let mut addcoded: &str="j";
      encoded_text.push_str(addcoded);
     }
     if letter=='q'{
      let mut addcoded: &str="n";
      encoded_text.push_str(addcoded);
     }
     if letter=='r'{
      let mut addcoded: &str="r";
      encoded_text.push_str(addcoded);
     }
     if letter=='s'{
      let mut addcoded: &str="v";
      encoded_text.push_str(addcoded);
     }
     if letter=='t'{
      let mut addcoded: &str="z";
      encoded_text.push_str(addcoded);
     }
     if letter=='u'{
      let mut addcoded: &str="w";
      encoded_text.push_str(addcoded);
     }
     if letter=='v'{
      let mut addcoded: &str="s";
      encoded_text.push_str(addcoded);
     }
     if letter=='w'{
      let mut addcoded: &str="o";
      encoded_text.push_str(addcoded);
     }
     if letter=='x'{
      let mut addcoded: &str="t";
      encoded_text.push_str(addcoded);
     }
     if letter=='y'{
      let mut addcoded: &str="x";
      encoded_text.push_str(addcoded);
     }
     if letter=='z'{
      let mut addcoded: &str="y";
      encoded_text.push_str(addcoded);
     }
         
   }

     count=count+1;
    
   } 
   
   println!("{}",encoded_text)
}

fn decoding(mut coded_text: String,iterations: i32){
   let mut count=0;
   let mut decoded_text: String = "".to_owned();
   while count<iterations{
      println!("Print {} iterations",count);
      for letter in coded_text.chars(){ 
         if letter=='a'{
            let mut addcoded: &str="a";
            decoded_text.push_str(addcoded);
           }
           if letter=='b'{
            let mut addcoded: &str="b";
            decoded_text.push_str(addcoded);
           }
           if letter=='c'{
            let mut addcoded: &str="f";
            decoded_text.push_str(addcoded);
           }
           if letter=='d'{
            let mut addcoded: &str="g";
            decoded_text.push_str(addcoded);
           }
           if letter=='e'{
            let mut addcoded: &str="o";
            decoded_text.push_str(addcoded);
           }
           if letter=='f'{
            let mut addcoded: &str="c";
            decoded_text.push_str(addcoded);
           }
           if letter=='g'{
            let mut addcoded: &str="e";
            decoded_text.push_str(addcoded);
           }
           if letter=='h'{
            let mut addcoded: &str="h";
            decoded_text.push_str(addcoded);
           }
           if letter=='i'{
            let mut addcoded: &str="n";
            decoded_text.push_str(addcoded);
           }
           if letter=='j'{
            let mut addcoded: &str="p";
            decoded_text.push_str(addcoded);
           }
           if letter=='k'{
            let mut addcoded: &str="d";
            decoded_text.push_str(addcoded);
           }
           if letter=='l'{
            let mut addcoded: &str="i";
            decoded_text.push_str(addcoded);
           }
           if letter=='m'{
            let mut addcoded: &str="m";
            decoded_text.push_str(addcoded);
           }
           if letter=='n'{
            let mut addcoded: &str="q";
            decoded_text.push_str(addcoded);
           }
           if letter=='o'{
            let mut addcoded: &str="w";
            decoded_text.push_str(addcoded);
           }
           if letter=='p'{
            let mut addcoded: &str="j";
            decoded_text.push_str(addcoded);
           }
           if letter=='q'{
            let mut addcoded: &str="l";
            decoded_text.push_str(addcoded);
           }
           if letter=='r'{
            let mut addcoded: &str="r";
            decoded_text.push_str(addcoded);
           }
           if letter=='s'{
            let mut addcoded: &str="v";
            decoded_text.push_str(addcoded);
           }
           if letter=='t'{
            let mut addcoded: &str="x";
            decoded_text.push_str(addcoded);
           }
           if letter=='u'{
            let mut addcoded: &str="k";
            decoded_text.push_str(addcoded);
           }
           if letter=='v'{
            let mut addcoded: &str="s";
            decoded_text.push_str(addcoded);
           }
           if letter=='w'{
            let mut addcoded: &str="u";
            decoded_text.push_str(addcoded);
           }
           if letter=='x'{
            let mut addcoded: &str="y";
            decoded_text.push_str(addcoded);
           }
           if letter=='y'{
            let mut addcoded: &str="z";
            decoded_text.push_str(addcoded);
           }
           if letter=='z'{
            let mut addcoded: &str="t";
            decoded_text.push_str(addcoded);
           }
               
      }   
     count=count+1;
   
   }
   println!("{}",decoded_text)
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
