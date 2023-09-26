
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::fmt::{self};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Note {
        pub id: usize,
        pub title: String,
        pub body: String,
        pub dob: DateTime<Utc>,
        pub tags: Vec<String>,
    }

    impl Note {
        pub fn new_empty(id: usize) -> Self {
            Self {
                id,
                title: "".to_string(),
                body: "".to_string(),
                dob: Utc::now(),
                tags: Vec::new(),
            }
        }

        pub fn new_base(id: usize, title: String, body: String) -> Self {
            Self {
                id,
                title,
                body,
                dob: Utc::now(),
                tags: Vec::new(),
            }
        }

        pub fn new_full(id: usize, title: String, body: String, tags: Vec<String>) -> Self {
            Self {
                id,
                title,
                body,
                dob: Utc::now(),
                tags,
            }
        }

        pub fn change_id(&mut self, new_id: usize) {
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

