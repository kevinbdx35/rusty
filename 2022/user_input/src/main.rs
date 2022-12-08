use std::io;

fn main() {
    let mut input :String = String::new();

    // Print a message to the user
    println!("Say Something");
    
    /*
    Check response and 
    act accordingly
    */
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
