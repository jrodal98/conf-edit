use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    path::PathBuf,
};
static READ: bool = true;
static WRITE: bool = false;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub editor: String,
    pub hook: String,
}

pub fn get_data() -> HashMap<String, Entry> {
    let conf_file = get_file(READ);
    serde_json::from_reader(&conf_file).expect("error opening config file")
}

pub fn write_data(data: HashMap<String, Entry>) {
    let conf_file = get_file(WRITE);
    serde_json::to_writer(&conf_file, &data).expect("Error writing to config");
}

fn get_file(read: bool) -> File {
    let path = dirs::home_dir()
        .expect("ERROR: $HOME is not set.")
        .join(".config/conf-edit/config.json");
    std::fs::create_dir_all(path.parent().unwrap()).expect("Error creating conf-edit directory");
    if let Ok(mut conf_file) = OpenOptions::new()
        .read(read)
        .write(true)
        .truncate(!read)
        .create_new(true)
        .open(&path)
    {
        let editor = match std::env::var("EDITOR") {
            Ok(val) => val,
            Err(_) => "vim".to_string(),
        };

        let entry = Entry {
            path: path.clone(),
            editor: editor.clone(),
            hook: "echo 'conf-edit config edited!'".to_string(),
        };

        let mut data = HashMap::new();
        data.insert("conf-edit".to_string(), entry);
        serde_json::to_writer(&conf_file, &data).expect("Error writing to config");
        conf_file.seek(SeekFrom::Start(0)).expect("Error going back to start of file");
        conf_file
    } else {
        OpenOptions::new()
            .read(read)
            .write(!read)
            .truncate(!read)
            .open(path)
            .expect("Error opening conf file")
    }
}
