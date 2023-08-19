mod note_m;
pub mod app {
    use super::note_m::note::Note;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use std::collections::HashMap;
    use std::fmt::{self};
    use std::fs::OpenOptions;
    use std::io::{Read, Write};
    use std::path::Path;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Notes {
        notes: HashMap<u64, Note>,
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

        fn update_tags(&mut self, note: Note) {
            for tag in note.tags.clone() {
                if !self.tags.contains(&tag) {
                    self.tags.push(tag);
                }
            }
        }

        pub fn add_note(&mut self, mut note: Note) {
            note.change_id(self.next_id);
            self.notes.insert(note.clone().id, note.clone());
            self.update_tags(note);
            self.next_id += 1;
        }

        pub fn get_note(&self, get_id: u64) -> Option<Note> {
            for (id, note) in &self.notes {
                if get_id == *id {
                    return Some(note.clone());
                }
            }
            return None;
        }

        pub fn print_all_notes(&self) {
            print!("{:#}", self);
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

    pub fn get_saved_data(file: &str) -> Notes {
        match OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(Path::new(file))
        {
            Ok(mut file) => {
                let mut json_data = String::new();
                match file.read_to_string(&mut json_data) {
                    Ok(_) => match serde_json::from_str(json_data.as_str()) {
                        Ok(data) => data,
                        Err(_) => panic!("Unable to deserialize"),
                    },
                    Err(_) => panic!("Unable to read data"),
                }
            }
            Err(_) => {
                panic!("Unable to create or open file")
            }
        }
    }

    pub fn save_data(notes: Notes, file: &str) {
        match serde_json::to_string_pretty(&notes) {
            Ok(json_data) => {
                match OpenOptions::new()
                    .append(true)
                    .create(true)
                    .read(true)
                    .open(Path::new(file))
                {
                    Ok(mut file) => {
                        file.write_all(json_data.as_bytes()).unwrap();
                    }
                    Err(_) => {
                        panic!("Unable to create or open file")
                    }
                }
            }
            Err(_) => panic!("Unable to serialize data"),
        }
    }
}
