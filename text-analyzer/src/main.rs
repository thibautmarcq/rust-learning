// Text Analyzer
//
// -- CAHIER DES CHARGES --
// Le nombre total de mots
// Le nombre de lignes
// Le mot le plus long (retourné comme slice &str sur le texte original, pas une copie)
// Les 3 mots les plus fréquents, avec leur nombre d'occurrences

use std::{collections::HashMap, fs::File, io::{self, Read, Write}, process::exit};

fn find_longest_word(text : &str) -> &str{
    let mut longest: &str = "";
    let mut max_len = 0;
    for word in text.split_whitespace(){
        let len = word.len();
        if len > max_len {
            max_len = len;
            longest = word;
        }
    }
    longest
}

fn main() {
    print!("Please enter the text file you'd like to analize > ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap(); 
    let file_name = file_name.trim().to_string();    
    println!("\nfile : '{file_name}'");
    let file_open_res = File::open(&file_name);
    
    let mut file = match file_open_res{
        Ok(f) => {println!("File ok"); f},
        Err(e) => {println!("Incorrect file : {}", e); exit(1)},
    };

    let mut buffer_file = String::new();
    if let Err(e) = file.read_to_string(&mut buffer_file) {
        println!("Impossible to read file ({e})");
        exit(1);
    }

    let mut nb_lines = 0;
    let mut nb_words = 0;
    let mut hash_map: HashMap<&str, usize> = HashMap::new();

    for line in buffer_file.lines(){
        // print!("{line}\n");
        nb_lines+=1;
        for word in line.split_whitespace(){
            // println!("{word}");
            nb_words += 1;
            *hash_map.entry(word).or_insert(0) += 1;
        }
    }

    let mut frequences: Vec<(&str, usize)> = hash_map.into_iter().collect();
    frequences.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0))); // tri stable en cas d'égalité

    println!("Stats - {file_name}");
    println!("- Number of lines : {nb_lines}");
    println!("- Number of words : {nb_words}");
    println!("- Longest word : '{}'", find_longest_word(&buffer_file));
    
    let top3: &[(&str, usize)] = &frequences[..frequences.len().min(3)];
    for (mot, count) in top3 {
        println!("- '{mot}' ({count} fois)");
    }
}
