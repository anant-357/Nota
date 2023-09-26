use pwhash::bcrypt;
use std::{io::{Read, Write}, path::Path, fs::OpenOptions, usize};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let pass = bcrypt::hash(password).unwrap();
        Self {
            username,
            password: pass,
        }
    }

    pub fn verify(&self, password: String) -> bool {
        bcrypt::verify(password, self.password.as_str())
    }
    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users {
    users: HashMap<String, User>,
}

impl Users {
    pub fn new() -> Self {
        Self { users: Vec::new(),}
    }

    pub fn exists(&self, username: String) -> isize {
        for user_index in 0..self.users.len() {
            if self.users[user_index].username == username {
                return user_index as isize
            }
        } 
        return -1
    }

    pub fn add_user(&mut self, username: String, password: String) {
       if self.exists(username.clone()) >= 0 {
            return
       }
       
       let user: User = User::new(username.clone(), password.clone());
        self.users.insert(0, user);

    }

    pub fn remove_user(&mut self, username: String) {
        let user_index = self.exists(username.clone());
        if user_index >= 0 {
            self.users.remove(user_index as usize);
        }
    }

    pub fn authenticate(&self, username: String, password: String) {
        
    }

    pub fn change_password(&mut self, username: String) {
        let user_index : isize = self.exists(username.clone());
        if user_index >= 0 {
            let mut old_password = String::new();
            println!("Enter old password: ");
            std::io::stdin().read_line(&mut old_password).expect("Failed to read old password");
            if self.users[user_index as usize].verify(old_password) == true {
                let mut new_password = String::new();
                println!("\nEnter new password: ");
                std::io::stdin().read_line(&mut new_password).expect("Failed to read new password");
                let mut new_password_confirm = String::new();
                println!("\nEnter password again: ");
                std::io::stdin().read_line(&mut new_password_confirm).expect("Failed to read confirmation password");
                if new_password == new_password_confirm {
                    self.users[user_index as usize].change_password(new_password);
                } else {
                    println!("Passwords don't match");
                }
            } else {
                println!("Wrong Password!");
            }
        } 
    }
}

pub fn get_users(file: &str) -> Users {
    match OpenOptions::new().append(true).create(true).read(true).open(Path::new(file)){
        Ok(mut file) => {
            let mut json_data = String::new();
            match file.read_to_string(&mut json_data) {
                Ok(_) => match serde_json:: from_str(json_data.as_str()) {
                    Ok(data) => data,
                    Err(_) => panic!("Unable to deserialize")
                },
                Err(_) => panic!("Unable to read data")
            }
        }
        Err(_) => panic!("Unable to create or open file")
    }

}

pub fn save_users(users: Users, file: &str){
    match serde_json::to_string_pretty(&users) {
        Ok(json_data) => {
            match OpenOptions::new().append(true).create(true).read(true).open(Path::new(file)){
                Ok(mut file) => {
                    file.write_all(json_data.as_bytes()).unwrap();
                }
                Err(_) => panic!("Unable to create or open file")
            }
        }
        Err(_) => panic!("Unable to serialize data"),
    }
}

