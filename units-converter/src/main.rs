mod conversion; // déclare le module, cherche conversion.rs ou conversion/mod.rs
use std::{io::{self, Write}};

fn read_stdin() -> f64{
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let var : f64 = buf.trim().parse().expect("The expected value is a number (float)");
    return var;
}

fn menu(){
    println!("---- CONVERTISSEUR UNITÉS ----");
    println!("1- Celsius > Fahrenheit");
    println!("2- Fahrenheit > Celsius");
    println!("-------------------------");
    print!(">");
    io::stdout().flush().unwrap(); // force l'écriture du char d'entrée
}

fn main() {
    menu();
    let var = read_stdin();
    if var == 1 as f64{
        print!("Température en Celsius > ");
        io::stdout().flush().unwrap();
        let var = read_stdin();
        let c = conversion::celsius_vers_fahrenheit(var);
        println!("{}°C = {}°F", var, c);
    } else if var == 2 as f64{
        print!("Température en Fahrenheit > ");
        io::stdout().flush().unwrap();
        let var = read_stdin();
        let c = conversion::fahrenheit_vers_celsius(var);
        println!("{}°F = {}°C", var, c);
    } else {
        println!("Erreur : Mauvaise entrée");
    }
}
