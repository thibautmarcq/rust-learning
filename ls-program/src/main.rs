use std::{env, fs, io, process::exit};

// Simple ls program to print the content of a folder
fn main(){
    let args : Vec<String> = env::args().collect();
    for arg in args.iter().enumerate() {
        println!("arg {}, arg {}", arg.0, arg.1);
    }

    let open_dir_res: Result<fs::ReadDir, io::Error>;
    let dir_path: String;
    if args.len() > 2 {
        println!("Usage : {} <path dossier>", args[0]);
        exit(1);
    } else if args.len() == 1 { // no arguments
        let input = io::stdin();
        let mut buffer = String::new();
        let _ = input.read_line(&mut buffer);
        let cleaned = buffer.trim_end_matches(['\n', '\r']).to_string();
        println!("Buffer lu : '{}'", cleaned);
        dir_path = cleaned.clone();
        open_dir_res = fs::read_dir(cleaned);
    } else { // one or many arguments (the first one only is used)
        dir_path = args[1].clone();
        open_dir_res = fs::read_dir(dir_path.clone());
    }

    let dir = match open_dir_res {
        Ok(dir) => {println!("Affichage du dossier {}", dir_path); dir },
        Err(error) => {
            println!("Path incorrect");
            println!("Erreur {}", error.to_string());
            exit(1);
        },
    };
    for entry in dir{
    match entry {
            Ok(sub) => println!("> {}", sub.file_name().into_string().unwrap()),
            Err(e) => println!("Erreur entrée sous-dossier : {}", e),
        }
    }
}