use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    #[structopt(short, long)]
    name: Option<String>,
    #[structopt(short, long)]
    editor: Option<String>,
    #[structopt(short, long)]
    script: Option<String>,
}

impl Add {
    fn run(&self, conf_file: File) {
        dbg!("Add");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Remove {
    name: String,
}

impl Remove {
    fn run(&self, conf_file: File) {
        dbg!("remove");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Execute {
    name: String,
}

impl Execute {
    fn run(&self, conf_file: File) {
        dbg!("execute");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Edit {
    name: String,
    #[structopt(short, long)]
    no_exec: bool,
}

impl Edit {
    fn run(&self, conf_file: File) {
        dbg!("edit");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct ListFiles {
    name: Option<String>,
}

impl ListFiles {
    fn run(&self, conf_file: File) {
        dbg!("ls");
        let json_text: JsonFile =
            serde_json::from_reader(&conf_file).expect("error opening config file");
        println!("{:?}", json_text);
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "add")]
    Add(Add),
    #[structopt(name = "rm")]
    Remove(Remove),
    #[structopt(name = "execute")]
    Execute(Execute),
    #[structopt(name = "edit")]
    Edit(Edit),
    #[structopt(name = "ls")]
    ListFiles(ListFiles),
}

impl Command {
    fn run(&self) {
        let conf_file = self.open_config();
        match self {
            Command::Add(command) => command.run(conf_file),
            Command::Remove(command) => command.run(conf_file),
            Command::Execute(command) => command.run(conf_file),
            Command::Edit(command) => command.run(conf_file),
            Command::ListFiles(command) => command.run(conf_file),
        }
    }

    fn open_config(&self) -> File {
        let path = dirs::home_dir()
            .expect("ERROR: $HOME is not set.")
            .join(".config/conf-edit/config");
        if Path::exists(&path) {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect("Error opening config file")
        } else {
            let editor = match std::env::var("EDITOR") {
                Ok(val) => val,
                Err(_) => "vim".to_string(),
            };

            let entry = Entry {
                path: path.to_str().unwrap().to_owned(),
                editor: editor.clone(),
                hook: "echo 'conf-edit config edited!'".to_owned(),
            };

            let mut data = HashMap::new();
            data.insert("conf-edit".to_owned(), entry);

            let f = JsonFile { editor, data };

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Error opening and/or creating config file");
            serde_json::to_writer(&file, &f).expect("Error writing to config");
            file.seek(SeekFrom::Start(0))
                .expect("Error rewinding file pointer"); // rewind file pointer back to beginning
            file
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    path: String,
    editor: String,
    hook: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonFile {
    editor: String,
    data: HashMap<String, Entry>,
}

fn run_app() -> Result<(), ()> {
    let opt = Command::from_args();
    opt.run();
    println!("{:#?}", opt);
    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
