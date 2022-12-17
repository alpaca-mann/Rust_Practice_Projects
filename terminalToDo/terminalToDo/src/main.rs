use std::io;
fn main() {
    loop{
   println!("Select from below");
   println!("Add new todo: 1");
   println!("Review todos: 2");
   println!("I'm done for now: 3");
    
   let mut menu_prompt=String::new();
   io::stdin()
   .read_line(&mut menu_prompt)
   .unwrap();

   let menu_prompt_trim = menu_prompt.trim();
    
   if menu_prompt_trim=="1" {
    println!("----");
    println!("Ok. What is new?");
        let mut new_todo:String = String::new();
        io::stdin()
            .read_line(&mut new_todo)
            .unwrap();
        
        let new_todo_trim:&str= new_todo.trim();

        println!("----");
        println!("New ToDo added.");
        println!("Content: {new_todo_trim}");
        println!("----");
        }
    else if menu_prompt_trim=="2" {
        println!("Work in progress.");
        break;
        }
        else if menu_prompt_trim=="3" {
            println!("Got it. See you around");
            break;
        }
    }
}

