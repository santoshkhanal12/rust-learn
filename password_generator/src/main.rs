use std::io;

fn main() {
    println!("Please select the mode Encrypt or decrypt");
    let mut mode=String::new();

    io::stdin()
       .read_line(&mut mode);
    mode=mode.to_lowercase();    //Converts all characters to lowercase
    println!("You chose mode {}", mode);

   
}
