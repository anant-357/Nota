pub mod note {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::fmt::{self};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Note {
        pub id: u64,
        pub title: String,
        pub body: String,
        pub dob: DateTime<Utc>,
        pub tags: Vec<String>,
    }

    impl Note {
        pub fn new_empty(id: u64) -> Self {
            Self {
                id: id,
                title: "".to_string(),
                body: "".to_string(),
                dob: Utc::now(),
                tags: Vec::new(),
            }
        }

        pub fn new_base(id: u64, title: String, body: String) -> Self {
            Self {
                id: id,
                title: title,
                body: body,
                dob: Utc::now(),
                tags: Vec::new(),
            }
        }

        pub fn new_full(id: u64, title: String, body: String, tags: Vec<String>) -> Self {
            Self {
                id: id,
                title: title,
                body: body,
                dob: Utc::now(),
                tags: tags,
            }
        }

        pub fn change_id(&mut self, new_id: u64) {
            self.id = new_id;
        }
    }

    impl fmt::Display for Note {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let width: usize = termsize::get().unwrap().cols as usize - 2;
            print!("{:-<width$}--\n", "");
            print!("|{:^width$}|\n", self.id);
            print!("|{:^width$}|\n", self.title);
            print!("|{:<width$}|\n", self.body);
            print!("{:-<width$}--\n", "");

            Ok(())
        }
    }
}
