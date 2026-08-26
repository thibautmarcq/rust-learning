//errors.rs


// #[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    SeparateurManquant { ligne: usize, contenu: String },
    PortInvalide { ligne: usize, source: std::num::ParseIntError },
    DebugInvalide { ligne: usize, valeur: String },
    ChampManquant(&'static str),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "erreur de lecture du fichier : {e}"),
            ConfigError::SeparateurManquant { ligne, contenu } => {
                write!(f, "ligne {ligne} : séparateur '=' manquant dans \"{contenu}\"")
            }
            ConfigError::PortInvalide { ligne, source } => {
                write!(f, "ligne {ligne} : port invalide ({source})")
            }
            ConfigError::DebugInvalide { ligne, valeur } => {
                write!(f, "ligne {ligne} : valeur debug invalide \"{valeur}\" (attendu true/false)")
            }
            ConfigError::ChampManquant(champ) => write!(f, "champ obligatoire manquant : {champ}"),
        }
    }
}