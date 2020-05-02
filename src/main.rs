use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

static READ: bool = true;
static WRITE: bool = false;

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    name: String,
    #[structopt(short, long)]
    editor: Option<String>,
    #[structopt(short, long)]
    script: Option<String>,
}

impl Add {
    fn run(&self) {
        dbg!("Add");

        let editor = match &self.editor {
            Some(x) => x.to_string(),
            None => match std::env::var("EDITOR") {
                Ok(val) => val,
                Err(_) => "vim".to_string(),
            },
        };

        let hook = match &self.script {
            Some(x) => x.to_string(),
            None => "".to_string(),
        };

        let entry = Entry {
            path: self.path.clone(),
            editor,
            hook,
        };

        let mut data = get_data();
        data.insert(self.name.clone(), entry);
        write_data(data);
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Remove {
    name: String,
}

impl Remove {
    fn run(&self) {
        dbg!("remove");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Execute {
    name: String,
}

impl Execute {
    fn run(&self) {
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
    fn run(&self) {
        dbg!("edit");
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct ListFiles {
    name: Option<String>,
}

impl ListFiles {
    fn run(&self) {
        dbg!("ls");
        println!("{:?}", get_data());
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
        match self {
            Command::Add(command) => command.run(),
            Command::Remove(command) => command.run(),
            Command::Execute(command) => command.run(),
            Command::Edit(command) => command.run(),
            Command::ListFiles(command) => command.run(),
        }
    }
}

fn get_file(read: bool) -> File {
    let path = dirs::home_dir()
        .expect("ERROR: $HOME is not set.")
        .join(".config/conf-edit/config.json");
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
        conf_file.seek(SeekFrom::Start(0));
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

fn get_data() -> HashMap<String, Entry> {
    let conf_file = get_file(READ);
    serde_json::from_reader(&conf_file).expect("error opening config file")
}

fn write_data(data: HashMap<String, Entry>) {
    let conf_file = get_file(WRITE);
    serde_json::to_writer(&conf_file, &data).expect("Error writing to config");
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    path: PathBuf,
    editor: String,
    hook: String,
}

fn run_app() -> Result<(), ()> {
    let opt = Command::from_args();
    opt.run();
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
