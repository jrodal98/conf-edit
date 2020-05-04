mod data_worker;
use crate::data_worker::{get_data, write_data, Entry};
use std::{fs::canonicalize, path::PathBuf, process::Command};
use structopt::StructOpt;

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
            path: canonicalize(&self.path).expect("Error canonicalizing path"),
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
        let mut data = get_data();
        if let Some(_) = data.remove(&self.name) {
            write_data(data);
        }
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct Execute {
    name: String,
}

impl Execute {
    fn run(&self) {
        match get_data().get(&self.name) {
            Some(entry) => {
                    execute(&entry.hook);
                }
            None => {
                    eprintln!("ERROR: {} does not exist. Add it with 'ce add'.", self.name);
                }
        }
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
        match get_data().get(&self.name) {
            Some(entry) => {
                    std::process::Command::new(&entry.editor)
                        .arg(&entry.path)
                        .status()
                        .expect("Failed when editing file");
                    if !self.no_exec {
                        execute(&entry.hook);
                    }
                }
            None => {
                    eprintln!("ERROR: {} is not a valid entry. Have you added it yet?", &self.name);
                }
        }
    }
}

fn execute(hook: &str) {
    if !hook.is_empty() {
        match shlex::split(hook) {
            Some(cmd) => {
                    if let Err(e) = Command::new(&cmd[0]).args(&cmd[1..]).status() {
                        eprintln!("ERROR: posthook failed: {}", e);
                    }
                }
            None => {
                    eprintln!("ERROR: posthook is not a valid shell command.");
                }
        }
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub struct ListFiles {
    // name: Option<String>,
}

impl ListFiles {
    fn run(&self) {
        println!("{:#?}", get_data());
    }
}

///////////////////////////////

#[derive(Debug, StructOpt)]
pub enum OptCommand {
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

impl OptCommand {
    fn run(&self) {
        match self {
            OptCommand::Add(command) => command.run(),
            OptCommand::Remove(command) => command.run(),
            OptCommand::Execute(command) => command.run(),
            OptCommand::Edit(command) => command.run(),
            OptCommand::ListFiles(command) => command.run(),
        }
    }
}

fn run_app() -> Result<(), ()> {
    let opt = OptCommand::from_args();
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
