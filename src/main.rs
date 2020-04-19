use std::path::PathBuf;
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
    fn run(&self) {
        dbg!("Add");
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
