// Config parser

//main.rs

use std::{io::{self, Write}, process::exit};

use crate::config::parser::parse_file;

mod config;
mod errors;


fn main() {
    print!("Please enter the config you'd like to parse > ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap(); 
    let file_name = file_name.trim().to_string();
    let test = parse_file(&file_name);
    match test {
        Err(e) => {println!("Erreur pendant le parsing de la config \n> {}", e); exit(1)},
        Ok(conf) => println!("Config lue : {}, {}, {}", conf.nom, conf.port, conf.debug),
    };
    
}
