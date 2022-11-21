use clap::Parser;
// use crate::data;

use rustyline::error::ReadlineError;
use rustyline::{Editor};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // filename: String,
}

pub struct Context {
    // pub filename : String,
    // pub data : data::Database,
}

pub fn exec() -> Result<(), impl std::fmt::Debug> {
    let args = Args::parse();

    // let data = data::Database::open(&args.filename).unwrap();

    let mut context = Context { 
        // filename : args.filename,
        // data,
    };


    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        // TODO : use clap for command parsing ?
        match readline {
            Ok(line) => {
                let lineclone = line.clone();
                let mut tokens = lineclone.split_whitespace();  
                match tokens.next() {
                    None => (),
                    // Load an anise file
                    Some("load") => {
                        let filename = tokens.next().unwrap();
                        println!("Loading {}", filename);
                        todo!();
                        rl.add_history_entry(line.as_str());
                    },
                    Some("write") => {
                        todo!();
                    }
                    Some(_) => println!("Unknown command")
                }
                
            },
            Err(ReadlineError::Interrupted) => {
                // println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                // println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    println!("Saving history...");
    rl.save_history("history.txt")
}