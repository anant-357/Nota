use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt::{self};
use std::fs;
use std::path::Path;
mod NoteM;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notes {
    notes: HashMap<u64, NoteM::Note>,
    tags: Vec<String>,
    next_id: u64,
}

impl Notes {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
            tags: Vec::new(),
            next_id: 1,
        }
    }

    fn update_tags(&mut self, note: NoteM::Note) {
        for tag in note.tags.clone() {
            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }
    }

    pub fn add_note(&mut self, mut note: NoteM::Note) {
        note.change_id(self.next_id);
        self.notes.insert(note.clone().id, note.clone());
        self.update_tags(note);
        self.next_id += 1;
    }

    pub fn get_note(&self, get_id: u64) -> Option<NoteM::Note> {
        for (id, note) in &self.notes {
            if get_id == *id {
                return Some(note.clone());
            }
        }
        return None;
    }
}

impl fmt::Display for Notes {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_id, note) in &self.notes {
            print!("{:^}\n\n", note);
        }
        Ok(())
    }
}

fn main() {
    let app_path = Path::new("data");
    match fs::metadata(app_path) {
        Ok(_) => println!("File exists!"),
        Err(_) => println!("File does not exist!"),
    }
    // let mut file = match File::create(&app_path) {
    //     Err(why) => panic!("couldn't create {}: {}", app_path.display(), why),
    //     Ok(file) => file,
    // };
    // let mut app: Notes = serde_json::from_str("{\"notes\":{\"2\":{\"id\":2,\"title\":\"Hello2\",\"body\":\"Body2\",\"dob\":\"2023-08-16T17:33:50.732040450Z\",\"tags\":[\"chem\",\"shreya\"]},\"1\":{\"id\":1,\"title\":\"Hello\",\"body\":\"Body\",\"dob\":\"2023-08-16T17:33:50.732027036Z\",\"tags\":[\"bio\",\"phy\"]}},\"tags\":[\"bio\",\"phy\",\"chem\",\"shreya\"],\"next_id\":3}").unwrap();
    // let t = vec![String::from("bio"), String::from("phy")];
    // app.add_note(NoteM::Note::new_full(
    //     1,
    //     "Hello".to_string(),
    //     "Body".to_string(),
    //     t,
    // ));
    // let t2 = vec![String::from("chem"), String::from("shreya")];

    // app.add_note(NoteM::Note::new_full(
    //     1,
    //     "Hello2".to_string(),
    //     "Body2".to_string(),
    //     t2,
    // ));
    // let serialized_app = serde_json::to_string(&app).unwrap();

    // println!("{}\n{}", app, app.get_note(2).unwrap());
    // println!("{:#?}", serialized_app);
}
