//parser.rs

use std::{fs::File, io::Read};

use crate::errors::ConfigError;

pub struct Config { pub nom: String, pub port: u16, pub debug: bool }

pub fn parse_file(file_name: &str) -> Result<Config, ConfigError> {
    let file_name = file_name.trim();
    let mut file = File::open(file_name)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut nom: Option<String> = None;
    let mut port: Option<u16> = None;
    let mut debug: Option<bool> = None;

    for (i, line) in buffer.lines().enumerate() {
        let numero_ligne = i + 1;
        if line.trim().is_empty() {
            continue;
        }

        let (cle, valeur) = line.split_once('=').ok_or(ConfigError::SeparateurManquant {
            ligne: numero_ligne,
            contenu: line.to_string(),
        })?;
        let cle = cle.trim();
        let valeur = valeur.trim();

        match cle {
            "nom" => nom = Some(valeur.to_string()),
            "port" => {
                port = Some(valeur.parse::<u16>().map_err(|source| ConfigError::PortInvalide {
                    ligne: numero_ligne,
                    source,
                })?);
            }
            "debug" => {
                debug = Some(match valeur {
                    "true" => true,
                    "false" => false,
                    _ => return Err(ConfigError::DebugInvalide {
                        ligne: numero_ligne,
                        valeur: valeur.to_string(),
                    }),
                });
            }
            _ => {} // clé inconnue : ignorée (ou à traiter en erreur si vous préférez être strict)
        }
    }

    Ok(Config {
        nom: nom.ok_or(ConfigError::ChampManquant("nom"))?,
        port: port.ok_or(ConfigError::ChampManquant("port"))?,
        debug: debug.ok_or(ConfigError::ChampManquant("debug"))?,
    })
}