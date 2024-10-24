pub mod engine;
pub mod args_cli;

use clap::Parser;
use crate::args_cli::Flags;

use crate::engine::{
    env::Env,
    dump::Dump,
};

use figlet_rs::FIGfont;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("DumpSync");
    println!("{}", figure.unwrap());

    Env::new();

    let dbname = if Flags::parse().database != None {
        Flags::parse().database.unwrap()
    } else {
        Env::get_var("DB_NAME")
    };

    let interval = if Flags::parse().interval != None {
        Flags::parse().interval.unwrap()
    } else {
        Env::get_var_u64("DUMP_INTERVAL")
    };

    let backup_path = if Flags::parse().folder != None {
        Flags::parse().folder.unwrap()
    } else {
        Env::get_var("BACKUP_PATH")
    };

    let user = if Flags::parse().user != None {
        Flags::parse().user.unwrap()
    } else {
        Env::get_var("DB_USER")
    };

    let password = if Flags::parse().password != None {
        Flags::parse().password.unwrap()
    } else {
        Env::get_var("DB_PASSWORD")
    };
    
    Dump::new(&user, &password, &dbname, &backup_path, interval).make_dump();
}
