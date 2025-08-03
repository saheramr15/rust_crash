use std::io;
use std::collections::HashMap;
 #[derive(Debug)]
struct Task {
    id:String,
    description:String,
    priority:String,
    status:String
}

fn main() {
     
     let mut choice= String::new();
     let mut tasks: HashMap<String, Task> = HashMap::new();

        loop {
            println!("Please enter your choice :\n1)Add a task \n2)List all tasks\n3)Mark a task as completed\n4)Delete a task\n5)Exit");
             choice.clear(); // clear input buffer before reading again
             io::stdin()
            .read_line(&mut choice)  
            .expect("Failed to read line");  
            match choice.trim().parse::<u32>() {
                
            Ok(num) => {
                if num == 1 {
                    let mut id = String::new();
                    let mut desc = String::new();
                    let mut priority = String::new();
                    let status = String::from("Pending");

                    println!("Enter task ID: ");
                     io::stdin()
                       .read_line(&mut id)  
                        .expect("Failed to read line");  
                                      
                    println!("Enter task description: ");                   
                     io::stdin()
                       .read_line(&mut desc)  
                        .expect("Failed to read line");  
                   
                    println!("Enter task priority: ");                   
                     io::stdin()
                       .read_line(&mut priority)  
                        .expect("Failed to read line");  
                   
                       tasks.insert(
                             id.to_string(), // Use the ID as key
                            Task {
                                id: id,
                                description: desc,
                                priority: priority,
                                status,
                            },
                );
                    println!("Task added successfully.");   
     
                } else if num ==2 {
                    println!("{:?}", tasks);
                   
                }else if num == 3 {
                    let mut id = String::new();
                    println!("Enter task ID: ");
                    io::stdin()
                       .read_line(&mut id)  
                        .expect("Failed to read line");
                    
                    if let Some(task) = tasks.get_mut( &id) {
                        task.status ="completed".to_string();
                    }

                } else if num == 4 {
                    let mut id = String::new();
                    println!("Enter task ID: ");
                    io::stdin()
                       .read_line(&mut id)  
                        .expect("Failed to read line");

                    if let Some(removed_task) = tasks.remove(&id) {
                            
                        } else {
                            println!("Task with ID {} not found.", id);
                    }
                   
                } else if num == 5 {
                    println!("Exiting...");
                    break;
                } else {
                    println!("Invalid choice. ");
                    
                }
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        } 
    }
} 