mod notes;
mod user;
use std::{io::Result, usize};
use notes::{Notes, save_data, get_saved_data};
use user::Users;

fn menu(){
    print!("0. Quit\n");
    print!("1. Add Note\n");
    print!("2. Delete Note\n");
    print!("3. Display Note\n");
    print!("4. Display All Notes\n");
    let mut choice_buf : String = String::new();
    std::io::stdin().read_line(&mut choice_buf).expect("Failed to read input!");
    let choice : usize = choice_buf.trim().parse().expect("Input is not an integer!");
    match choice {
        0 => std::process::exit(0),
        1 => println!("1\n"),
        2 => println!("2\n"),
        3 => println!("3\n"),
        4 => println!("4\n"),
        _ => println!("_\n"),
    }
}

fn run(_notes : Notes) -> Result<()>{
    loop {
        menu();
    }
}

fn print_help(){
    println!("Usage: nota [OPTION] ...");
    println!("A simple note taking cli/tui tool. Stores small notes persistently with encryption.");
    println!("Starts tui/interactive mode if flags not used.");
    println!("\t-a, --add\t\t\t add a new note");
    println!("\t-d, --delete=ID\t\t\t delete note by ID");
    println!("\t-l, --list\t\t\t list all notes");
    println!("\t-g, --get=ID\t\t\t print note by ID");
    println!("\t-h, --help\t\t\t print this menu");
}

fn add_short_note(body: &String, mut _notes: Notes){
    println!("{}",body);
    _notes.add_note(body.to_string(),String::from("Title"));
    save_data(_notes.clone(), "data.txt"); 
}

fn delete_note(id: usize, _notes: Notes){
    println!("deleting id :{}",id);
}

fn get_note(id: usize, _notes: Notes){
    println!("getting note :{}",id);
}

fn handle_arguments(arguments: Vec<String>, notes:Notes){
    if arguments.len() == 1 {
       let _ = run(notes.clone());
        
    } else {
        if arguments.len() == 2 {
            if &arguments[1] == "-l" || &arguments[1] == "--list" {
                println!("Listing!!");
            } else {
                print_help();
            }
        } else {
            match arguments[1].as_str() {
                "-a" | "--add" => add_short_note(&arguments[2..].join(" "), notes.clone()) ,
                "-d" | "--delete" => delete_note(arguments[2].trim().parse().expect("Illegal id format!"), notes.clone()) ,
                "-g" | "--get" => get_note(arguments[2].trim().parse().expect("Illegal id format!"), notes.clone()) ,
                _ => print_help(), 
            }
        } 
    }

}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut users: Users = user::get_users("users.txt");
    let mut user: String = String::new();
    println!("Enter Username to continue :");
    std::io::stdin().read_line(&mut user).expect("Unable to read user\n");
    if users.exists(user) == -1 {
        panic!("Cannot find user\n");
    }
    println!("Enter Password :");
    let mut password: String = String::new();
    std::io::stdin().read_line(&mut password).expect("Unable to read password\n");

    users.authenticate(user, password);

    let notes: Notes = get_saved_data("data.txt");
    handle_arguments(args, notes);
    //let _ = run(notes);
    Ok(())
}
