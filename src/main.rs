mod app;
use app::app::Notes;
use serde_json;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn get_saved_data() -> app::app::Notes {
    match OpenOptions::new().write(true).create(true).open("data") {
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

fn main() {
    let notes: Notes = get_saved_data();
    println!("{:#}", notes);
}

// let mut app: Notes = serde_json::from_str().unwrap();
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
