mod note_m;
pub mod app {
    use super::note_m::note::Note;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fmt::{self};

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
    }

    impl fmt::Display for Notes {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (_id, note) in &self.notes {
                print!("{:^}\n\n", note);
            }
            Ok(())
        }
    }
}
