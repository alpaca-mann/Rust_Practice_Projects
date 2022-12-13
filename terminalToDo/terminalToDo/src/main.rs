use std::io;
fn main() {
//loop {
   println!("Add your ToDo");

   let mut new_todo:String = String::new();

   io::stdin()
       .read_line(&mut new_todo)
       .expect("Sorry, try again.");

    
    println!("Do you wish to add more? y = Yes. n = No.");
    //let mut test:String = String::new();
//}   
}
